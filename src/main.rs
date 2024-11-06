use rand::Rng;

// Функция для генерации случайного вектора длиной n со значениями [10..99]
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// Функция для нахождения минимальной суммы смежных пар в Vec<i32>
fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_indices = (0, 1);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_indices = (i, i + 1);
        }
    }

    (min_sum, min_indices.0, min_indices.1)
}

// Функция для вывода вектора и минимальной суммы смежных элементов
fn display_vector_with_min_sum(data: &[i32]) {
    // Вывод индексов
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}. ", i);
    }
    println!();

    // Вывод данных
    print!("data:   ");
    for value in data {
        print!("{:>3}, ", value);
    }
    println!();

    // Нахождение минимальной суммы и индексов
    let (min_sum, index1, index2) = min_adjacent_sum(data);

    // Вывод нижней строки с указателями
    print!("indexes: ");
    for i in 0..data.len() {
        if i == index1 {
            print!("\\__ ");
        } else if i == index2 {
            print!("__/ ");
        } else {
            print!("     ");
        }
    }
    println!();

    // Вывод минимальной суммы и индексов
    println!("min adjacent sum={}+{}={} at indexes:{},{}",
             data[index1], data[index2], min_sum, index1, index2);
}

fn main() {
    let data = gen_random_vector(20);
    display_vector_with_min_sum(&data);
}
