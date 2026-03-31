use macroquad::prelude::*;
use std::collections::HashMap;

const NODE_RADIUS: f32 = 45.0;
const FONT_SIZE: u16 = 24;
const SEARCH_INTERVAL: f32 = 2.0; // Segundos entre passos da animação

#[derive(Clone, Copy, PartialEq)]
enum SearchPhase {
    SearchingLocal,
    TraversingUp,
    FoundResult,
}

#[derive(Clone)]
struct Node {
    name: String,
    pos: Vec2,
    property: Option<(&'static str, bool)>,
    parent_name: Option<String>,
}

enum SearchState {
    Idle,
    Visualizing {
        path: Vec<String>,
        current_step: usize,
        timer: f32,
        found_at: Option<String>,
        result: Option<bool>,
        phase: SearchPhase,
    },
}

struct App {
    nodes: HashMap<String, Node>,
    state: SearchState,
    msg: String,
    initialized: bool,
}

impl App {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            state: SearchState::Idle,
            msg: "Bem-vindo! Clique em 'Canário' ou 'Avestruz' para iniciar a aula.".to_string(),
            initialized: false,
        }
    }

    fn init_nodes(&mut self) {
        if self.initialized { return; }
        
        let w = screen_width();
        let h = screen_height();

        let mut nodes = HashMap::new();
        
        // Estrutura Semântica: Animal <- Pássaro <- (Canário, Avestruz)
        nodes.insert("Animal".to_string(), Node {
            name: "Animal".to_string(),
            pos: vec2(w / 2.0, h * 0.15),
            property: None,
            parent_name: None,
        });

        nodes.insert("Pássaro".to_string(), Node {
            name: "Pássaro".to_string(),
            pos: vec2(w / 2.0, h * 0.4),
            property: Some(("voa", true)),
            parent_name: Some("Animal".to_string()),
        });

        nodes.insert("Canário".to_string(), Node {
            name: "Canário".to_string(),
            pos: vec2(w * 0.3, h * 0.7),
            property: None, // Aqui acontece Herança
            parent_name: Some("Pássaro".to_string()),
        });

        nodes.insert("Avestruz".to_string(), Node {
            name: "Avestruz".to_string(),
            pos: vec2(w * 0.7, h * 0.7),
            property: Some(("voa", false)), // Aqui acontece Sobrescrita
            parent_name: Some("Pássaro".to_string()),
        });

        self.nodes = nodes;
        self.initialized = true;
    }

    fn start_search(&mut self, start_node: &str) {
        let mut path = Vec::new();
        let mut current = start_node.to_string();
        let mut result = None;
        let mut found_at = None;

        // Algoritmo de Busca Semântica Tradicional
        loop {
            path.push(current.clone());
            if let Some(node) = self.nodes.get(&current) {
                if let Some((prop, val)) = node.property {
                    if prop == "voa" {
                        result = Some(val);
                        found_at = Some(current.clone());
                        break;
                    }
                }
                if let Some(ref parent) = node.parent_name {
                    current = parent.clone();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        self.state = SearchState::Visualizing {
            path,
            current_step: 0,
            timer: 0.0,
            found_at,
            result,
            phase: SearchPhase::SearchingLocal,
        };
    }

    fn update(&mut self) {
        self.init_nodes();

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let m_vec = vec2(mx, my);
            
            let mut clicked = None;
            for (name, node) in &self.nodes {
                if m_vec.distance(node.pos) < NODE_RADIUS {
                    clicked = Some(name.clone());
                    break;
                }
            }

            if let Some(name) = clicked {
                if name == "Canário" || name == "Avestruz" {
                    self.start_search(&name);
                }
            }
        }

        if let SearchState::Visualizing { 
            ref path, 
            ref mut current_step, 
            ref mut timer, 
            ref found_at, 
            result, 
            ref mut phase,
        } = self.state {
            *timer += get_frame_time();

            let current_node_name = &path[*current_step];
            let is_target = found_at.as_ref().map_or(false, |f| f == current_node_name);

            if *timer > SEARCH_INTERVAL {
                *timer = 0.0;
                if is_target {
                    *phase = SearchPhase::FoundResult;
                    let res_str = if result.unwrap() { "TRUE (Voa)" } else { "FALSE (Não voa)" };
                    self.msg = format!("PROFESSOR: Propriedade encontrada em {}! Resultado final: {}.", current_node_name, res_str);
                } else if *current_step < path.len() - 1 {
                    *current_step += 1;
                    *phase = SearchPhase::TraversingUp;
                }
            } else {
                match phase {
                    SearchPhase::SearchingLocal => {
                        self.msg = format!("IA: Verificando se o nó '{}' possui a propriedade 'voa'...", current_node_name);
                    }
                    SearchPhase::TraversingUp => {
                        self.msg = format!("PROFESSOR: {} não possui 'voa'. Subindo via herança...", path[*current_step - 1]);
                    }
                    SearchPhase::FoundResult => {}
                }
            }
        }
    }

    fn draw(&self) {
        clear_background(Color::from_rgba(15, 15, 25, 255));

        // Gradiente de fundo
        draw_circle(screen_width()/2.0, screen_height()/2.0, screen_width(), Color::from_rgba(25, 25, 50, 255));

        // 1. Desenhar Arestas (IS-A)
        for node in self.nodes.values() {
            if let Some(ref parent_name) = node.parent_name {
                if let Some(parent) = self.nodes.get(parent_name) {
                    draw_line(node.pos.x, node.pos.y, parent.pos.x, parent.pos.y, 4.0, Color::from_rgba(80, 100, 200, 150));
                    let mid = (node.pos + parent.pos) / 2.0;
                    draw_text("is-a", mid.x + 10.0, mid.y, 18.0, DARKGRAY);
                }
            }
        }

        // 2. Desenhar Nós
        for node in self.nodes.values() {
            let mut is_active = false;
            let mut is_target_found = false;

            if let SearchState::Visualizing { ref path, current_step, found_at: ref _found_at, phase, .. } = self.state {
                if path[current_step] == node.name {
                    is_active = true;
                    if phase == SearchPhase::FoundResult {
                        is_target_found = true;
                    }
                }
            }

            let base_color = if is_target_found { GREEN } else if is_active { GOLD } else { DARKBLUE };
            
            if is_active {
                draw_circle(node.pos.x, node.pos.y, NODE_RADIUS + 8.0, Color::from_rgba(255, 255, 215, 50));
            }

            draw_circle(node.pos.x, node.pos.y, NODE_RADIUS, base_color);
            draw_circle_lines(node.pos.x, node.pos.y, NODE_RADIUS, 3.0, WHITE);

            let t_measure = measure_text(&node.name, None, FONT_SIZE, 1.0);
            draw_text(&node.name, node.pos.x - t_measure.width/2.0, node.pos.y + 5.0, FONT_SIZE as f32, WHITE);

            if let Some((p, v)) = node.property {
                let color = if v { SKYBLUE } else { ORANGE };
                draw_text(&format!("{}: {}", p, v), node.pos.x - 30.0, node.pos.y + NODE_RADIUS + 25.0, 18.0, color);
            }
        }

        // 3. Cursor de Busca Animado
        if let SearchState::Visualizing { ref path, current_step, timer, phase, .. } = self.state {
            if phase != SearchPhase::FoundResult {
                let node = &self.nodes[&path[current_step]];
                let pulse = (timer * 5.0).sin() * 5.0;
                draw_circle_lines(node.pos.x, node.pos.y, NODE_RADIUS + 12.0 + pulse, 5.0, YELLOW);
            }
        }

        // 4. Painel Educativo (Legendas Dinâmicas)
        let rect_h = 100.0;
        draw_rectangle(20.0, screen_height() - rect_h - 20.0, screen_width() - 40.0, rect_h, Color::from_rgba(0, 0, 0, 220));
        draw_rectangle_lines(20.0, screen_height() - rect_h - 20.0, screen_width() - 40.0, rect_h, 2.0, GRAY);
        
        draw_text("AULA DE IA - NARRATIVA DO ALGORITMO:", 40.0, screen_height() - rect_h, 18.0, GOLD);
        draw_text(&self.msg, 40.0, screen_height() - 50.0, 22.0, WHITE);

        // Header
        draw_text("Simulador de Herança e Sobrescrita", 30.0, 50.0, 32.0, WHITE);
        draw_text("Clique em um nó folha para iniciar a busca", 30.0, 80.0, 18.0, LIGHTGRAY);
    }
}

#[macroquad::main("AI Search Simulator")]
async fn main() {
    let mut app = App::new();
    loop {
        app.update();
        app.draw();
        next_frame().await
    }
}
