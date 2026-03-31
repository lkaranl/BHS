use macroquad::prelude::*;
use std::collections::HashMap;

const NODE_RADIUS: f32 = 45.0;

const CLR_BG: Color = Color::new(0.04, 0.04, 0.08, 1.0);
const CLR_NODE: Color = Color::new(0.12, 0.15, 0.25, 0.95);
const CLR_HIGHLIGHT: Color = Color::new(1.0, 0.75, 0.0, 1.0);
const CLR_ACCENT: Color = Color::new(0.2, 0.6, 1.0, 1.0);
const CLR_SUCCESS: Color = Color::new(0.2, 0.9, 0.4, 1.0);
const CLR_TEXT: Color = Color::new(0.95, 0.95, 1.0, 1.0);

#[derive(Clone, Copy, PartialEq)]
enum Lesson { None, Canary, Ostrich, Whale, Bat, Rex }

#[derive(Clone, Copy, PartialEq)]
enum StepPhase { Analysing, ResultFail, Success }

struct Node {
    pos: Vec2,
    properties: Vec<(&'static str, bool)>,
    parent: Option<String>,
}

struct App {
    nodes: HashMap<String, Node>,
    active_lesson: Lesson,
    path: Vec<String>,
    current_idx: usize,
    phase: StepPhase,
    log: Vec<String>,
    target_prop: &'static str,
    initialized: bool,
}

impl App {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            active_lesson: Lesson::None,
            path: Vec::new(),
            current_idx: 0,
            phase: StepPhase::Analysing,
            log: Vec::new(),
            target_prop: "",
            initialized: false,
        }
    }

    fn init(&mut self) {
        if self.initialized { return; }
        let w = screen_width();
        let h = screen_height();
        let off_y = h * 0.15;
        let mut n = HashMap::new();

        n.insert("Vida".to_string(), Node { pos: vec2(w * 0.5, h * 0.1 + off_y), properties: vec![("respira", true)], parent: None });
        n.insert("Ave".to_string(), Node { pos: vec2(w * 0.25, h * 0.3 + off_y), properties: vec![("voa", true)], parent: Some("Vida".to_string()) });
        n.insert("Mamífero".to_string(), Node { pos: vec2(w * 0.75, h * 0.3 + off_y), properties: vec![("voa", false)], parent: Some("Vida".to_string()) });
        n.insert("Canário".to_string(), Node { pos: vec2(w * 0.12, h * 0.5 + off_y), properties: vec![], parent: Some("Ave".to_string()) });
        n.insert("Avestruz".to_string(), Node { pos: vec2(w * 0.38, h * 0.5 + off_y), properties: vec![("voa", false)], parent: Some("Ave".to_string()) });
        n.insert("Morcego".to_string(), Node { pos: vec2(w * 0.62, h * 0.5 + off_y), properties: vec![("voa", true)], parent: Some("Mamífero".to_string()) });
        n.insert("Baleia".to_string(), Node { pos: vec2(w * 0.88, h * 0.5 + off_y), properties: vec![], parent: Some("Mamífero".to_string()) });
        n.insert("Rex (Cão)".to_string(), Node { pos: vec2(w * 0.8, h * 0.65 + off_y), properties: vec![], parent: Some("Mamífero".to_string()) });

        self.nodes = n;
        self.initialized = true;
    }

    fn select_lesson(&mut self, lesson: Lesson) {
        self.active_lesson = lesson;
        self.current_idx = 0;
        self.phase = StepPhase::Analysing;
        self.log.clear();
        
        let (start, prop) = match lesson {
            Lesson::Canary => ("Canário", "voa"),
            Lesson::Ostrich => ("Avestruz", "voa"),
            Lesson::Whale => ("Baleia", "voa"),
            Lesson::Bat => ("Morcego", "voa"),
            Lesson::Rex => ("Rex (Cão)", "respira"),
            _ => ("", ""),
        };

        if start == "" { return; }
        self.target_prop = prop;
        
        let mut p = Vec::new();
        let mut curr = start.to_string();
        loop {
            p.push(curr.clone());
            if let Some(node) = self.nodes.get(&curr) {
                if node.properties.iter().any(|(k, _)| *k == prop) { break; }
                if let Some(ref pa) = node.parent { curr = pa.clone(); } else { break; }
            } else { break; }
        }
        self.path = p;
        self.log.push(format!("OBJETIVO: Encontrar '{}' começando em {}", prop, start));
        self.log.push(format!("-> Analisando nó: {}", start));
    }

    fn next_step(&mut self) {
        if self.active_lesson == Lesson::None { return; }

        match self.phase {
            StepPhase::Analysing => {
                let name = &self.path[self.current_idx];
                if let Some(node) = self.nodes.get(name) {
                    if let Some((_, val)) = node.properties.iter().find(|(k, _)| *k == self.target_prop) {
                        self.log.insert(0, format!("! ACHEI ! '{}'={} em {}.", self.target_prop, val, name));
                        self.phase = StepPhase::Success;
                    } else {
                        self.log.insert(0, format!("? Não tem '{}' em {}.", self.target_prop, name));
                        self.phase = StepPhase::ResultFail;
                    }
                }
            }
            StepPhase::ResultFail => {
                if self.current_idx < self.path.len() - 1 {
                    self.current_idx += 1;
                    let name = &self.path[self.current_idx];
                    self.log.insert(0, format!("-> Subindo para o pai: {}", name));
                    self.phase = StepPhase::Analysing;
                } else {
                    self.log.insert(0, "x FIM: Não encontrado em lugar nenhum.".to_string());
                    self.phase = StepPhase::Success; // Só para parar
                }
            }
            StepPhase::Success => {} // Nada a fazer
        }
    }

    fn update(&mut self) {
        self.init();
        if is_key_pressed(KeyCode::Key1) { self.select_lesson(Lesson::Canary); }
        if is_key_pressed(KeyCode::Key2) { self.select_lesson(Lesson::Ostrich); }
        if is_key_pressed(KeyCode::Key3) { self.select_lesson(Lesson::Whale); }
        if is_key_pressed(KeyCode::Key4) { self.select_lesson(Lesson::Bat); }
        if is_key_pressed(KeyCode::Key5) { self.select_lesson(Lesson::Rex); }
        if is_key_pressed(KeyCode::Escape) { self.active_lesson = Lesson::None; }
        if is_key_pressed(KeyCode::Space) { self.next_step(); }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            if mx > screen_width() - 220.0 && my > screen_height() - ph_h() - 100.0 && my < screen_height() - ph_h() {
                self.next_step();
            }
        }
    }

    fn draw(&self) {
        clear_background(CLR_BG);

        for (name, node) in &self.nodes {
            if let Some(ref p_name) = node.parent {
                if let Some(parent) = self.nodes.get(p_name) {
                    let active = self.active_lesson != Lesson::None && self.path.contains(name) && self.path.contains(p_name);
                    draw_line(node.pos.x, node.pos.y, parent.pos.x, parent.pos.y, 2.0, if active { CLR_HIGHLIGHT } else { Color::new(0.2,0.2,0.3,0.7) });
                }
            }
        }

        for (name, node) in &self.nodes {
            let focus = self.active_lesson != Lesson::None && self.path.get(self.current_idx) == Some(&name.to_string());
            draw_circle(node.pos.x, node.pos.y, NODE_RADIUS, if focus { CLR_HIGHLIGHT } else { CLR_NODE });
            draw_circle_lines(node.pos.x, node.pos.y, NODE_RADIUS, 2.0, WHITE);
            let t_sz = measure_text(name, None, 18, 1.0);
            draw_text(name, node.pos.x - t_sz.width/2.0, node.pos.y + 6.0, 18.0, CLR_TEXT);
            let mut py = node.pos.y + NODE_RADIUS + 20.0;
            for (p, v) in &node.properties {
                draw_text(&format!("{}:{}", p, if *v { "S" } else { "N" }), node.pos.x - 20.0, py, 14.0, if *v { GREEN } else { RED });
                py += 15.0;
            }
        }

        let log_w = 560.0;
        let log_h = 110.0;
        let log_x = (screen_width() - log_w) / 2.0;
        let log_y = 15.0;
        draw_rectangle(log_x, log_y, log_w, log_h, Color::new(0.0, 0.0, 0.1, 0.95));
        draw_rectangle_lines(log_x, log_y, log_w, log_h, 2.0, CLR_ACCENT);
        draw_text("PASSO A PASSO DA IA:", log_x + 15.0, log_y + 25.0, 18.0, CLR_HIGHLIGHT);
        let mut ly = log_y + 50.0;
        for msg in self.log.iter().take(3) {
            let c = if msg.contains("ACHEI") { CLR_SUCCESS } else if msg.contains("Não tem") { GRAY } else { WHITE };
            draw_text(msg, log_x + 15.0, ly, 16.0, c);
            ly += 20.0;
        }

        if self.active_lesson != Lesson::None && self.phase != StepPhase::Success {
            let btn_x = screen_width() - 200.0;
            let btn_y = screen_height() - ph_h() - 90.0;
            draw_rectangle(btn_x, btn_y, 180.0, 60.0, CLR_ACCENT);
            draw_rectangle_lines(btn_x, btn_y, 180.0, 60.0, 2.0, WHITE);
            draw_text("PRÓXIMO", btn_x + 40.0, btn_y + 38.0, 22.0, WHITE);
            
            let tut = match self.phase {
                StepPhase::Analysing => "AIA: Clique para VERIFICAR se o nó tem a característica.",
                StepPhase::ResultFail => "AIA: Não encontrado aqui! Clique para SUBIR para o pai.",
                _ => "",
            };
            draw_text(tut, 30.0, btn_y + 35.0, 18.0, YELLOW);
        }

        let ph = ph_h();
        draw_rectangle(0.0, screen_height() - ph, screen_width(), ph, Color::new(0.0, 0.0, 0.0, 1.0));
        let mut ix = 20.0;
        for itm in ["[1] Canário", "[2] Avestruz", "[3] Baleia", "[4] Morcego", "[5] Rex", "[ESC]"] {
            draw_text(itm, ix, screen_height() - 20.0, 16.0, CLR_ACCENT);
            ix += screen_width() / 6.0;
        }
    }
}

fn ph_h() -> f32 { 45.0 }

#[macroquad::main("IA Semantic Search - Granular Mode")]
async fn main() {
    let mut app = App::new();
    loop { app.update(); app.draw(); next_frame().await }
}
