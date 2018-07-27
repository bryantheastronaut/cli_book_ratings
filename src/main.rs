use std::collections::HashMap;
use std::io;

type ReviewList = HashMap<String, Vec<Review>>;

struct Review {
    score: f32,
    review_text: String,
}

fn print_reviews(reviews: &ReviewList) {
    for (book, scores) in reviews {
        println!("{}:", book);
        for r in scores {
            println!("{}: {}", r.score, r.review_text);
        };
        println!("--------------------------------------");
    };
}

fn add_review(mut reviews: ReviewList) -> ReviewList {
    println!("What is the title of the book you want to review?");
    let mut book_title = String::new();
    io::stdin().read_line(&mut book_title).expect("Cannot read input 0.o");
    let book_title = book_title.trim().to_string();
    println!("What score do you give it? (Must be between 0.0 and 5.0)");
    let mut score = String::new();
    io::stdin().read_line(&mut score).expect("Cannot read input");
    let score = score.trim().parse::<f32>().unwrap();
    let mut review_text = String::new();
    println!("What do you have to say about it?");
    io::stdin().read_line(&mut review_text).expect("Cannot read user input");
    let review_text = review_text.trim().to_string();
    let new_review = Review {
        score, review_text
    };
    reviews.entry(book_title).or_insert(vec![]).push(new_review);
    print_reviews(&reviews);
    reviews
}

fn main() {
    println!("Welcome to the best CLI book review system available!");
    let mut reviews: ReviewList = HashMap::new();
    let dune_review = Review {
        score: 4.5,
        review_text: "This book was great. I liked the spice :)".to_string(),
    };
    reviews.entry("Dune".to_string()).or_insert(vec![]).push(dune_review);
    print_reviews(&reviews);
    let mut user_input = String::new();
    println!("Do you want to add a review?");
    println!("1. Add review");
    io::stdin().read_line(&mut user_input)
        .expect("Cannot read user input");
    match user_input.trim().parse() {
        Ok(1) => add_review(reviews),
        _ => reviews,
    };
}