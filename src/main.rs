// https://rust-cli.github.io/book/index.html
use std::path::PathBuf;
use std::collections::HashMap;

use structopt::StructOpt;

//OBS!! Merk at std::error::Error er en trait, mens std::io::Error er en struct!!
use blackboard_course_manager::BBCourseManager;
use blackboard_course_manager::bb_course::BBCourse;
use blackboard_course_manager::bb_course::bb_content::time_utils;

#[derive(StructOpt, Debug)]
#[structopt(name = "Blackboard Course Manager", about = "A tool for managing Blackboard courses")]
enum Bbcm {
    #[structopt(about="Register new course")]
    Register,

    #[structopt(about="View registered courses")]
    Courses,

    #[structopt(about="Download course file tree")]
    Tree {
        #[structopt(
            name="course-alias",
            help="Alias of course",
        )]
        course_alias: String,
        
        #[structopt(
            short,
            long,
            help="Force download of non-updated content",
        )]
        overwrite: bool,

        #[structopt(
            short,
            long,
            help="Unzip zip files after download",
        )]
        unzip: bool,
    },

    #[structopt(about="Download course file trees for all registered courses")]
    Trees {
        #[structopt(
            short,
            long,
            help="Force download of non-updated content",
        )]
        overwrite: bool,

        #[structopt(
            short,
            long,
            help="Unzip zip files after download",
        )]
        unzip: bool,
    },

    #[structopt(about="View course announcements")]
    Announcements {
        #[structopt(
            name="course-alias",
            help="Alias of course",
        )]
        course_alias: String,

        #[structopt(
            short,
            long,
            name="limit",
            help="Limit announcements",
        )]
        limit: Option<usize>,

        #[structopt(
            short,
            long,
            name="offset",
            help="Offset announcements",
        )]
        offset: Option<usize>,
    },

    #[structopt(about="Remove registered course")]
    Remove {
        #[structopt(
            name="course-alias",
            help="Alias of course",
        )]
        course_alias: String,
    },

    #[structopt(about="Remove all registered courses")]
    Reset
}

fn main() {
    let domain = std::env::var("BBCM_DOMAIN").expect("Please set BBCM_DOMAIN environment variable.");
    let out_dir = PathBuf::from(&std::env::var("BBCM_OUT_DIR").expect("Error: Environment variable BBCM_OUT_DIR is not set")); 
    let work_dir = std::env::var("BBCM_WORK_DIR").map(|val| PathBuf::from(&val)).unwrap_or_else(|_| std::env::temp_dir().join("bbcm_work"));

    let manager = BBCourseManager::new(&domain, &out_dir, &work_dir);
    let mut courses: HashMap<String, BBCourse> = manager.load_courses().into_iter().map(|course| (course.alias.clone(), course)).collect();

    match Bbcm::from_args() {
        Bbcm::Register => {
            let course = BBCourse::register(&manager);
            courses.insert(course.alias.clone(), course);
        },

        Bbcm::Courses => {
            for course in courses.values() {
                course.view();
            }
        },

        Bbcm::Tree {
            course_alias,
            overwrite,
            unzip,
        } => {
            if let Some(course) = courses.get_mut(&course_alias) {
                if let Ok(download_size) = course.download_course_content_tree(None, None, overwrite, unzip) {
                    println!("Downloaded a total of {:.1} MB.", download_size/1000000.0);
                    course.last_tree_download = time_utils::now();
                } 
            } else {
                eprintln!("Course with alias {} not found.", course_alias);
            }
        },

        Bbcm::Trees {
            overwrite,
            unzip,
        } => {
            for (alias, course) in &mut courses {
                println!("Downloading tree for {}.", alias);
                if let Ok(download_size) = course.download_course_content_tree(None, None, overwrite, unzip) {
                    println!("Downloaded a total of {:.1} MB.", download_size/1000000.0);
                    course.last_tree_download = time_utils::now();
                } 
            } 
        },

        Bbcm::Announcements {
            course_alias,
            limit,
            offset,
        } => {
            if let Some(course) = courses.get(&course_alias) {
                course.view_course_announcements(limit, offset).unwrap();
            } else {
                eprintln!("Course with alias {} not found.", course_alias);
            }
        },

        Bbcm::Remove {
            course_alias,
        } => {
            if courses.remove(&course_alias).is_none() {
                eprintln!("Course with alias {} not found.", course_alias);
            }
        },

        Bbcm::Reset => {
            courses.clear();
        }
    }

    manager.save_courses(&courses.into_iter().map(|t| t.1).collect::<Vec<BBCourse>>()[..]);
}
