// https://rust-cli.github.io/book/index.html
use std::path::Path;

use structopt::StructOpt;

//OBS!! Merk at std::error::Error er en trait, mens std::io::Error er en struct!!
use blackboard_course_manager::blackboard_course_manager::BBCourseManager;

#[derive(StructOpt, Debug)]
#[structopt(name = "Blackboard Course Manager", about = "A tool for managing Blackboard courses")]
enum Bcm {
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
    },

    #[structopt(about="View course content")]
    Contents {
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
    },

    #[structopt(about="View course content attachments")]
    Attachments {
        #[structopt(
            name="course-alias",
            help="Alias of course",
        )]
        course_alias: String,

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
    },

    #[structopt(about="Download course files")]
    DownloadFiles {
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
    }

    //Session

}

fn main() {

    let mut course_manager = BBCourseManager::new(
        Path::new(&std::env::var("BBCM_OUT_DIR").expect("Error: Environment variable BBCM_OUT_DIR is not set")), 
        Path::new(&std::env::var("BBCM_WORK_DIR").unwrap_or_else(|_| String::from(".\\work")))
    );

    let args = Bcm::from_args();

    match args {
        Bcm::Register => {
            course_manager.register_course();
        },

        Bcm::Remove {
            course_alias,
        } => {
            course_manager.remove_course(&course_alias);
        },

        Bcm::Reset => {
            course_manager.remove_all_courses();
        }

        Bcm::Courses => {
            course_manager.view_courses();
        },

        Bcm::DownloadTree {
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

        Bcm::Announcements {
            course_alias,
            limit,
            offset,
        } => {
            course_manager.view_course_announcements(&course_alias, limit, offset).unwrap();
        },

        Bcm::Contents {
            course_alias,
            title,
        } => {
            course_manager.view_course_content(&course_alias, title).unwrap();
        },

        Bcm::Attachments {
            course_alias,
            filename,
            mimetype,
        } => {
            course_manager.view_course_attachments(&course_alias, filename, mimetype).unwrap();
        },

        Bcm::DownloadFiles {
            course_alias,
            title,
            filename,
            mimetype,
            unzip,
            overwrite,
        } => {
            if let Ok(download_size) = course_manager.download_course_content_attachments(&course_alias, title, filename, mimetype, unzip, overwrite) {
                println!("Downloaded a total of {} bytes.", download_size);
            }
        }
    
    }

}
