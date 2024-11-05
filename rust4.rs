fn main() {
    const SIZE: usize = 5; // Розмір ромба

    // Верхня частина ромба
    for i in 0..SIZE {
        let stars = 2 * i + 1; // Кількість зірочок
        let spaces = SIZE - i - 1; // Кількість пробілів перед зірочками

        // Виведення пробілів і зірочок
        println!("{:width$}{}", "", "*".repeat(stars), width = spaces);
    }

    // Нижня частина ромба
    for i in (0..SIZE-1).rev() {
        let stars = 2 * i + 1; // Кількість зірочок
        let spaces = SIZE - i - 1; // Кількість пробілів перед зірочками

        // Виведення пробілів і зірочок
        println!("{:width$}{}", "", "*".repeat(stars), width = spaces);
    }
}
