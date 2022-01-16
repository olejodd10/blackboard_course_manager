// https://rust-cli.github.io/book/index.html
use std::path::{Path, PathBuf};

use structopt::StructOpt;

//OBS!! Merk at std::error::Error er en trait, mens std::io::Error er en struct!!
use blackboard_course_manager::blackboard_course_manager::BBCourseManager;

#[derive(StructOpt, Debug)]
#[structopt(name = "Blackboard Course Manager", about = "A tool for managing Blackboard courses")]
enum Bbcm {
    #[structopt(about="Register new course")]
    Register,

    #[structopt(about="Remove registered course")]
    Remove {
        #[structopt(
            name="course-alias",
            help="Alias of course",
        )]
        course_alias: String,
    },

    #[structopt(about="Remove all registered courses")]
    Reset,

    #[structopt(about="View registered courses")]
    Courses,

    #[structopt(about="Download course file tree")]
    DownloadTree {
        #[structopt(
            name="course-alias",
            help="Alias of course",
        )]
        course_alias: String,

        #[structopt(
            short,
            long,
            help="Content title filter"
        )]
        title: Option<String>, 
        
        #[structopt(
            short,
            long,
            help="Filename filter"
        )]
        filename: Option<String>, 
        
        #[structopt(
            short,
            long,
            help="Mimetype filter"
        )]
        mimetype: Option<String>,
        
        #[structopt(
            short,
            long,
            help="Unzip zip files after download",
        )]
        unzip: bool,
        
        #[structopt(
            short,
            long,
            help="Overwrite files",
        )]
        overwrite: bool,
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
    }
}

fn main() {

    let mut course_manager = BBCourseManager::new(
        Path::new(&std::env::var("BBCM_OUT_DIR").expect("Error: Environment variable BBCM_OUT_DIR is not set")), 
        &std::env::var("BBCM_WORK_DIR").map(|val| PathBuf::from(&val)).unwrap_or_else(|_| std::env::temp_dir().join("bbcm_work"))
    );

    let args = Bbcm::from_args();

    match args {
        Bbcm::Register => {
            course_manager.register_course();
        },

        Bbcm::Remove {
            course_alias,
        } => {
            course_manager.remove_course(&course_alias);
        },

        Bbcm::Reset => {
            course_manager.remove_all_courses();
        }

        Bbcm::Courses => {
            course_manager.view_courses();
        },

        Bbcm::DownloadTree {
            course_alias,
            title,
            filename,
            mimetype,
            unzip,
            overwrite,
        } => {
            if let Ok(download_size) = course_manager.download_course_content_tree(&course_alias, title, filename, mimetype, unzip, overwrite) {
                println!("Downloaded a total of {} bytes.", download_size);
            }
        },

        Bbcm::Announcements {
            course_alias,
            limit,
            offset,
        } => {
            course_manager.view_course_announcements(&course_alias, limit, offset).unwrap();
        }
    }
}
