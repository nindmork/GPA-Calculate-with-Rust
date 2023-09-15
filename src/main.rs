use std::io;

fn main() {
    let mut lab_score = String::new();
    let mut exam_score = String::new();

    println!("กรุณาป้อนคะแนน Lab: ");
    io::stdin()
        .read_line(&mut lab_score)
        .expect("การอ่านค่าผิดพลาด");

    let lab_score: f64 = lab_score
        .trim()
        .parse()
        .expect("การแปลงค่าผิดพลาด");

    println!("กรุณาป้อนคะแนน Exam: ");
    io::stdin()
        .read_line(&mut exam_score)
        .expect("การอ่านค่าผิดพลาด");

    let exam_score: f64 = exam_score
        .trim()
        .parse()
        .expect("การแปลงค่าผิดพลาด");

    // คำนวณคะแนนรวมตามน้ำหนักที่กำหนด
    let total_score = (lab_score * 0.4) + (exam_score * 0.6);

    // ใช้ match เพื่อกำหนดเกรด
    let grade = match total_score {
        90.0..=100.0 => "เกรด A",
        80.0..=89.0 => "เกรด B",
        70.0..=79.0 => "เกรด C",
        60.0..=69.0 => "เกรด D",
        _ => "เกรด F",
    };

    println!("คะแนนทั้งหมด: {}", total_score);
    println!("เกรด: {}", grade);
}
