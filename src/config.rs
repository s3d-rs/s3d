macro_rules! env_config {
    ($env:ident optional) => {
        lazy_static::lazy_static! {
            pub static ref $env : Option<String> = std::env::var(stringify!($env)).ok();
        }
    };
    ($env:ident required) => {
        lazy_static::lazy_static! {
            pub static ref $env : String = std::env::var(stringify!($env)).unwrap();
        }
    };
    ($env:ident default $default_val:expr) => {
        lazy_static::lazy_static! {
            pub static ref $env : String = std::env::var(stringify!($env)).unwrap_or(($default_val).to_string());
        }
    };
}

env_config!(HOME required);
env_config!(S3D_LOCAL_DIR default ".s3d");
env_config!(S3D_ENDPOINT default "http://localhost:33333");

env_config!(S3_ENDPOINT optional);
env_config!(S3_ACCESS_KEY optional);
env_config!(S3_SECRET_KEY optional);

env_config!(S3D_WRITE_QUEUE default "false");
env_config!(S3D_WRITE_QUEUE_DIR default format!("{}/write_queue", *S3D_LOCAL_DIR));
env_config!(S3D_WRITE_QUEUE_FILTER optional);
env_config!(S3D_WRITE_QUEUE_MAX_SIZE optional);
env_config!(S3D_WRITE_QUEUE_MAX_FILES optional);
env_config!(S3D_WRITE_QUEUE_MAX_AGE optional);

env_config!(S3D_READ_CACHE default "false");
env_config!(S3D_READ_CACHE_DIR default format!("{}/read_cache", *S3D_LOCAL_DIR));
env_config!(S3D_READ_CACHE_FILTER optional);
env_config!(S3D_READ_CACHE_MAX_SIZE optional);
env_config!(S3D_READ_CACHE_MAX_FILES optional);
env_config!(S3D_READ_CACHE_MAX_AGE optional);

env_config!(S3D_SYNC_FOLDER default "false");
env_config!(S3D_SYNC_FOLDER_DIR default format!("{}/sync_folder", *S3D_LOCAL_DIR));
env_config!(S3D_SYNC_FOLDER_FILTER optional);
env_config!(S3D_SYNC_FOLDER_MAX_SIZE optional);
env_config!(S3D_SYNC_FOLDER_MAX_FILES optional);
env_config!(S3D_SYNC_FOLDER_MAX_AGE optional);

env_config!(S3D_FUSE_MOUNT default "false");
env_config!(S3D_FUSE_MOUNT_DIR default format!("{}/fuse_mount", *S3D_LOCAL_DIR));
