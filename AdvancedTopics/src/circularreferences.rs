use std::rc::Rc;
use std::cell::RefCell;

// 1) struct Student<'a> {
// 2) struct Student {
// 3) Adding another struct which manages the relation between them
struct Student {
    name: String,
    // 1) courses: Vec<&'a Course<'a>>             // Lifetime required if you add a reference
    // 2) courses: Vec<Rc<RefCell<Course>>>
}

// 1)  impl <'a> Student<'a> {
// 2) impl Student {
    // 1)  fn new(name: &str) -> Student<'a> {
//     fn new(name: &str) -> Student {
//         Student {
//             name: name.into(),                  // .into()  Trait to convert to another type
//             courses: Vec::new()
//         }
//     }
// }
// 3)
impl Student {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform.enrollments.iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

// 1) struct Course<'a> {
// 2) struct Course {
// 3) Adding another struct which manages the relation between them
struct Course {
    name: String,
    // 1) students: Vec<&'a Student<'a>>              // Lifetime required if you add a reference
    // 2) students: Vec<Rc<RefCell<Student>>>
}

// 1) impl <'a> Course<'a> {
// impl Course {
    // 1) fn new(name: &str) -> Course<'a> {
    // fn new(name: &str) -> Course {
    //     Course {
    //         name: name.into(),                  // .into()  Trait to convert to another type
    //         students: Vec::new()
    //     }
    // }

    // 1) Without using neither Rc nor RefCell --> We get error
    // fn add_student(&'a mut self, student: &'a mut Student<'a>) {
    //     student.courses.push(self);             // Asked for as mutable
    //     self.students.push(student);            // error[E0502]     self.students is mutable
    // }
    // 2) Using Rc and RefCell
    // fn add_student(course: Rc<RefCell<Course>>, student: Rc<RefCell<Student>>) {
    //     // borrow_mut()     because it's necessary mutable references
    //     // .clone()         create another reference
    //     student.borrow_mut().courses.push(course.clone());
    //     course.borrow_mut().students.push(student);
    // }
// }

// 3) Adding another struct which manages the relation between previous struct
// Lifetime required to indicate because the attributes are indicated by reference
struct Enrollment<'a> {
    student: &'a Student,                   // Reference to other struct
    course: &'a Course
}

impl<'a> Enrollment<'a> {
    fn new(course: &'a Course, student: &'a Student) -> Enrollment<'a >{
        Enrollment{course, student}         // Unnecessary to specify the arguments, because they match
    }
}

// 3) Adding another struct which stores the list of enrollments
struct Platform<'a > {
    enrollments: Vec<Enrollment<'a>>
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a >{
        Platform{
            enrollments: Vec::new()                 // Initialized by default as empty
        }
    }
    fn enroll(&mut self, student: &'a Student, course: &'a Course){             // self         Points to Platform struct
        self.enrollments.push(
            Enrollment::new(course, student)
        )
    }
}

pub fn demo() {
    // 1) Without using neither Rc nor RefCell --> We get error
    // let alfredo = Student::new("Alfredo");
    // let course = Course::new("Rust Course");

    // If course is dropped firstly, alfredo has got still a reference to a course already dropped
    // If alfredo is dropped firstly, course has got still a reference which it's not longer existence
    // course.add_student(alfredo);

    // 2) With Rc and RefCell
    // let alfredo = Rc::new(
    //     RefCell::new(
    //         Student::new("Alfredo")
    //     )
    // );
    // let jose = Rc::new(
    //     RefCell::new(
    //         Student::new("Jose")
    //     )
    // );
    //
    // let course = Course::new("Rust Course");
    // let wrappedCourse = Rc::new(
    //     RefCell::new(
    //         course
    //     )
    // );

    // error[E0382]   If we invoke several times passing wrapperCourse
    // Course::add_student(wrappedCourse, alfredo);            // Invoke a function which requires as argument the own struct

    // Course::add_student(wrappedCourse.clone(), alfredo);       // Ugly code, each time needed .clone()
    // Course::add_student(wrappedCourse, jose);

    // 3)
    let alfredo = Student {
        name: "Alfredo".into()
    };
    let course = Course {
        name: "Rust Course".into()
    };
    let mut p = Platform::new();
    p.enroll(&alfredo, &course);

    for c in alfredo.courses(p) {
        println!{"Alfredo is taking {}", c};
    }

}