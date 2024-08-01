pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFA500, 
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    pub fn render(&mut self, estado: &mut Vec<Vec<bool>>) {
        let width = self.width;
        let height = self.height;
        let mut nuevo_estado = estado.clone();

        for y in 0..height {
            for x in 0..width {
                let mut vecinos_vivos = 0;
                for dy in [-1, 0, 1].iter() {
                    for dx in [-1, 0, 1].iter() {
                        if *dx == 0 && *dy == 0 {
                            continue;
                        }

                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                            if estado[ny as usize][nx as usize] {
                                vecinos_vivos += 1;
                            }
                        }
                    }
                }
                if estado[y][x] {
                    if vecinos_vivos < 2 || vecinos_vivos > 3 {
                        nuevo_estado[y][x] = false; 
                    }
                } else {
                    if vecinos_vivos == 3 {
                        nuevo_estado[y][x] = true;
                    }
                }
                if nuevo_estado[y][x] {
                    self.point(x, y);
                }
            }
        }
        *estado = nuevo_estado;
    }
}
