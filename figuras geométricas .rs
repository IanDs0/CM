use std::f64::consts::PI;

trait FiguraGeometrica {
    fn area(&self) -> f64;
}

struct Quadrado {
    lado: f64
}

impl FiguraGeometrica for Quadrado {
    fn area(&self) -> f64 {
        self.lado * self.lado
    }
}

struct TrianguloEquilatero {
    base: f64,
    altura: f64
}

impl FiguraGeometrica for TrianguloEquilatero {
    fn area(&self) -> f64 {
        (self.base * self.altura) / 2.0
    }
}

struct Circulo {
    raio: f64
}

impl FiguraGeometrica for Circulo {
    fn area(&self) -> f64 {
        PI * self.raio * self.raio
    }
}

fn main() {
    let quadrado = Quadrado { lado: 5.0 };
    let triangulo = TrianguloEquilatero { base: 5.0, altura: 5.0 };
    let circulo = Circulo { raio: 5.0 };

    let figuras = vec![&quadrado as &dyn FiguraGeometrica, &triangulo as &dyn FiguraGeometrica, &circulo as &dyn FiguraGeometrica];

    for figura in figuras {
        let nome = match figura {
            &Quadrado { .. } => "Quadrado",
            &TrianguloEquilatero { .. } => "Triângulo Equilátero",
            &Circulo { .. } => "Círculo"
        };
        println!("Tipo da figura: {} - Área: {}", nome, figura.area());
    }
}
