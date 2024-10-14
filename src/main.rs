fn draw_triangle(base_width: usize, max_width: usize) -> Vec<String> {
    let mut triangle = Vec::new();
    for i in 0..(base_width / 2 + 1) {
        let stars = "*".repeat(2 * i + 1);
        let spaces = " ".repeat((max_width - (2 * i + 1)) / 2);
        triangle.push(format!("{}{}", spaces, stars));
    }
    triangle
}

fn draw_tree(triangle_count: usize) -> Vec<String> {
    let mut tree = Vec::new();

    let max_width = 3 + 2 * (triangle_count - 1);

    for t in 0..triangle_count {
        let single_star = format!("{:^width$}", "*", width = max_width);
        tree.push(single_star);

        let base_width = 3 + 2 * t;
        let triangle = draw_triangle(base_width, max_width);
        tree.extend(triangle);
    }

    tree
}

fn print_tree(triangle_count: usize) {
    let tree = draw_tree(triangle_count);
    for line in tree {
        println!("{}", line);
    }
}

fn main() {
    let triangle_count = 5;  // Кількість трикутників
    print_tree(triangle_count);
}
