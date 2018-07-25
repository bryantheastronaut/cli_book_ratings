use std::collections::HashMap;
use std::io;

type ReviewsMap = HashMap<String, Vec<f32>>;

fn handle_new_review(review_map: ReviewsMap) -> ReviewsMap {
    let mut new_review_map = review_map;
    println!("Are you adding a new book with a score or just a new score?");
    println!("1. New book with score\n2. New score only");
    let mut response = String::new();
    io::stdin().read_line(&mut response)
        .expect("Failed to read user input");
    match response.trim().parse() {
        Ok(1) => {
            new_review_map = add_book(new_review_map);
        },
        Ok(2) => {
            new_review_map = add_score(new_review_map);
        },
        _ => (),
    };
    new_review_map
}

fn print_reviews(reviews: &ReviewsMap) {
    for (book, scores) in reviews {
        let count = scores.len() as f32;
        let mut average = scores.iter().fold(0.0, |sum, val| sum + val);
        average = average / count;
        println!("{}. {} reviews. Average score: {:.1}", book, count, average);
    }
}

fn add_book(reviews: ReviewsMap) -> ReviewsMap {
    println!("inside add book");
    reviews
}

fn add_score(reviews: ReviewsMap) -> ReviewsMap {
    let mut book_number = String::new();
    let books_len = reviews.keys().len() as i32;
    println!("What book are you adding a score to?");
    for (i, book) in reviews.keys().enumerate() {
        println!("{}. {}", i+1, book);
    }
    io::stdin().read_line(&mut book_number)
        .expect("Failed to read line");
    match book_number.trim().parse::<i32>() {
        Ok(n) if n < books_len && n > 0 => {
            println!("Whats ur score? (Must be between 0.0 and 5.0)");
            let mut score = String::new();
            io::stdin().read_line(&mut score)
                .expect("Cannot read input");
            let score: f64 = score.trim().parse::<f64>().unwrap();
            if score > 5.0 || score < 0.0 {
                println!("Invalid score. Must be between 0.0 and 5.00");
            } else {
                println!("Nice!");
                // score is good, push to vec.
            }
        },
        _ => (),
    };
    reviews
}

fn main() {
    let mut reviews = HashMap::new();
    reviews.insert(String::from("Dune"), vec![4.8, 5.0, 4.75, 3.9]);
    reviews.insert(String::from("Neuromancer"), vec![4.4, 3.9, 4.65, 4.4]);
    reviews.insert(String::from("The Bible"), vec![1.0, 0.4, 1.45]);
    print_reviews(&reviews);
    println!("Add a review? Y/N");
    let mut should_add_review = String::new();
    io::stdin().read_line(&mut should_add_review)
        .expect("Failed to read user input");
    match should_add_review.trim().to_uppercase().as_ref() {
        "Y" | "YES" => {
            reviews = handle_new_review(reviews);
            print_reviews(&reviews);
        },
        _ => println!("Thanks! bye"),
    };
}