use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "Blackboard Course Manager", about = "A tool for managing Blackboard courses")]
pub enum Bbcm {
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
    },

    #[structopt(about="Download course file trees for all registered courses")]
    Trees {
        #[structopt(
            short,
            long,
            help="Force download of non-updated content",
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

    #[structopt(about="View gradebook columns for all registered courses")]
    Gradebooks {
        #[structopt(
            short,
            long,
            help="Show past deadlines",
        )]
        past: bool,
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
