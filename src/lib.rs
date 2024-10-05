use std::sync::{Arc, Mutex};

pub mod repository {
    use super::*;
    use log::info;

    pub mod blob {
        use log::info;
        use std::sync::{Arc, Mutex};

        pub struct Segment {
            pub(crate) index: u32,
            pub(crate) data: Vec<u8>,
            pub(crate) compressed: bool,
        }

        impl Segment {
            pub fn new(index: u32, data: Vec<u8>, compressed: bool) -> Self {
                if data.is_empty() {
                    panic!("Dữ liệu không được để trống.");
                }
                info!(
                    "Creating new Segment with index: {}, compressed: {}",
                    index, compressed
                );
                Segment {
                    index,
                    data,
                    compressed,
                }
            }

            pub fn get_data(&self) -> &Vec<u8> {
                info!("Getting data for Segment with index: {}", self.index);
                &self.data
            }

            pub fn set_data(&mut self, data: Vec<u8>) {
                if data.is_empty() {
                    panic!("Dữ liệu không được để trống.");
                }
                info!("Setting new data for Segment with index: {}", self.index);
                self.data = data;
            }
        }
    }

    pub mod pull {
        use log::info;

        #[derive(Debug)]
        pub enum Status {
            Open,
            Merged,
            Closed,
        }

        impl Status {
            pub fn describe(&self) -> &str {
                info!("Describing Status: {:?}", self);
                match self {
                    Status::Open => "Pull request đang mở",
                    Status::Merged => "Pull request đã được gộp",
                    Status::Closed => "Pull request đã bị đóng",
                }
            }
        }
    }

    pub mod change {
        use log::info;

        #[derive(Clone, Debug)]
        pub enum Type {
            Add,
            Modify,
            Delete,
        }

        impl Type {
            pub fn describe(&self) -> &str {
                info!("Describing Change Type: {:?}", self);
                match self {
                    Type::Add => "Thêm mới tệp hoặc thư mục",
                    Type::Modify => "Chỉnh sửa tệp hoặc thư mục",
                    Type::Delete => "Xóa tệp hoặc thư mục",
                }
            }
        }

        #[derive(Clone)]
        pub struct Summary {
            pub(crate) file_path: String,
            pub(crate) change_type: Type,
            pub(crate) summary: String,
        }

        impl Summary {
            pub fn new(file_path: String, change_type: Type, summary: String) -> Self {
                if file_path.is_empty() || summary.is_empty() {
                    panic!("Đường dẫn tệp và tóm tắt không được để trống.");
                }
                info!(
                    "Creating new Summary for file: {} with change type: {:?}",
                    file_path, change_type
                );
                Summary {
                    file_path,
                    change_type,
                    summary,
                }
            }

            pub fn get_file_path(&self) -> &String {
                info!("Getting file path for Summary: {}", self.file_path);
                &self.file_path
            }

            pub fn get_change_type(&self) -> &Type {
                info!(
                    "Getting change type for Summary of file: {}",
                    self.file_path
                );
                &self.change_type
            }

            pub fn get_summary(&self) -> &String {
                info!("Getting summary for file: {}", self.file_path);
                &self.summary
            }
        }

        #[derive(Clone)]
        pub struct Detail {
            pub(crate) line_number: u32,
            pub(crate) old_content: String,
            pub(crate) new_content: String,
        }

        impl Detail {
            pub fn new(line_number: u32, old_content: String, new_content: String) -> Self {
                if old_content.is_empty() || new_content.is_empty() {
                    panic!("Nội dung cũ và mới không được để trống.");
                }
                info!("Creating new Detail for line number: {}", line_number);
                Detail {
                    line_number,
                    old_content,
                    new_content,
                }
            }

            pub fn get_line_number(&self) -> u32 {
                info!("Getting line number for Detail: {}", self.line_number);
                self.line_number
            }

            pub fn get_old_content(&self) -> &String {
                info!("Getting old content for line number: {}", self.line_number);
                &self.old_content
            }

            pub fn get_new_content(&self) -> &String {
                info!("Getting new content for line number: {}", self.line_number);
                &self.new_content
            }
        }
    }

    pub mod issue {
        use log::info;

        #[derive(Debug)]
        pub enum Status {
            Open,
            InProgress,
            Resolved,
            Closed,
        }

        impl Status {
            pub fn describe(&self) -> &str {
                info!("Describing Status: {:?}", self);
                match self {
                    Status::Open => "Issue đang mở",
                    Status::InProgress => "Issue đang được xử lý",
                    Status::Resolved => "Issue đã được giải quyết",
                    Status::Closed => "Issue đã bị đóng",
                }
            }
        }

        pub struct Comment {
            pub(crate) author: String,
            pub(crate) content: String,
            pub(crate) timestamp: u64,
        }

        impl Comment {
            pub fn new(author: String, content: String, timestamp: u64) -> Self {
                if author.is_empty() || content.is_empty() {
                    panic!("Tác giả và nội dung bình luận không được để trống.");
                }
                info!(
                    "Creating new Comment by author: {} at timestamp: {}",
                    author, timestamp
                );
                Comment {
                    author,
                    content,
                    timestamp,
                }
            }

            pub fn get_author(&self) -> &String {
                info!("Getting author for Comment: {}", self.author);
                &self.author
            }

            pub fn get_content(&self) -> &String {
                info!("Getting content for Comment by author: {}", self.author);
                &self.content
            }

            pub fn get_timestamp(&self) -> u64 {
                info!("Getting timestamp for Comment by author: {}", self.author);
                self.timestamp
            }
        }
    }

    #[derive(Clone)]
    pub struct Commit {
        pub(crate) hash: String,
        pub(crate) author: String,
        pub(crate) message: String,
        pub(crate) timestamp: u64,
        pub(crate) changes: Vec<change::Summary>,
        pub(crate) detailed_changes: Vec<change::Detail>,
    }

    impl Commit {
        pub fn new(hash: String, author: String, message: String, timestamp: u64) -> Self {
            if hash.is_empty() || author.is_empty() || message.is_empty() {
                panic!("Hash, author và message không được để trống.");
            }
            info!(
                "Creating new Commit with hash: {} by author: {}",
                hash, author
            );
            Commit {
                hash,
                author,
                message,
                timestamp,
                changes: Vec::new(),
                detailed_changes: Vec::new(),
            }
        }

        pub fn add_change(&mut self, summary: change::Summary, detail: change::Detail) {
            info!("Adding change to Commit with hash: {}", self.hash);
            self.changes.push(summary);
            self.detailed_changes.push(detail);
        }

        pub fn get_hash(&self) -> &String {
            info!("Getting hash for Commit by author: {}", self.author);
            &self.hash
        }

        pub fn get_author(&self) -> &String {
            info!("Getting author for Commit with hash: {}", self.hash);
            &self.author
        }

        pub fn get_message(&self) -> &String {
            info!("Getting message for Commit with hash: {}", self.hash);
            &self.message
        }

        pub fn get_timestamp(&self) -> u64 {
            info!("Getting timestamp for Commit with hash: {}", self.hash);
            self.timestamp
        }

        pub fn get_changes(&self) -> &Vec<change::Summary> {
            info!("Getting changes for Commit with hash: {}", self.hash);
            &self.changes
        }

        pub fn get_detailed_changes(&self) -> &Vec<change::Detail> {
            info!(
                "Getting detailed changes for Commit with hash: {}",
                self.hash
            );
            &self.detailed_changes
        }
    }

    pub struct Blob {
        pub(crate) id: String,
        pub(crate) content_segments: Arc<Mutex<Vec<blob::Segment>>>,
        pub(crate) size: u64,
        pub(crate) recombine: bool,
    }

    impl Blob {
        pub fn new(id: String, size: u64) -> Self {
            if id.is_empty() || size == 0 {
                panic!("ID không được để trống và kích thước phải lớn hơn 0.");
            }
            info!("Creating new Blob with ID: {} and size: {}", id, size);
            Blob {
                id,
                content_segments: Arc::new(Mutex::new(Vec::new())),
                size,
                recombine: false,
            }
        }

        pub fn add_segment(&self, segment: blob::Segment) {
            let mut segments = self.content_segments.lock().unwrap();
            if segments.iter().any(|s| s.index == segment.index) {
                panic!("Segment with the same index already exists.");
            }
            info!(
                "Adding Segment with index: {} to Blob with ID: {}",
                segment.index, self.id
            );
            segments.push(segment);
        }

        pub fn recombine_segments(&self) -> Vec<u8> {
            if self.recombine {
                info!("Recombining segments for Blob with ID: {}", self.id);
                let segments = self.content_segments.lock().unwrap();
                segments.iter().flat_map(|seg| seg.data.clone()).collect()
            } else {
                info!("Recombine flag is not set for Blob with ID: {}", self.id);
                Vec::new()
            }
        }

        pub fn set_recombine(&mut self, recombine: bool) {
            info!(
                "Setting recombine flag for Blob with ID: {} to: {}",
                self.id, recombine
            );
            self.recombine = recombine;
        }

        pub fn get_recombine(&self) -> bool {
            info!("Getting recombine flag for Blob with ID: {}", self.id);
            self.recombine
        }
    }

    pub struct Metadata {
        pub created: u64,
        pub updated: u64,
        pub author: String,
    }

    impl Metadata {
        pub fn new(created: u64, updated: u64, author: String) -> Self {
            if author.is_empty() {
                panic!("Tác giả không được để trống.");
            }
            info!("Creating new Metadata for author: {}", author);
            Metadata {
                created,
                updated,
                author,
            }
        }

        pub fn get_created(&self) -> u64 {
            info!("Getting created timestamp for Metadata by author: {}", self.author);
            self.created
        }

        pub fn get_updated(&self) -> u64 {
            info!("Getting updated timestamp for Metadata by author: {}", self.author);
            self.updated
        }

        pub fn get_author(&self) -> &String {
            info!("Getting author for Metadata: {}", self.author);
            &self.author
        }
    }

    pub struct Branch {
        name: String,
        head: Arc<Mutex<Commit>>,
        commits: Arc<Mutex<Vec<Commit>>>,
        archived: bool,
        last_active: u64,
    }

    impl Branch {
        pub fn new(name: String, head: Commit, last_active: u64) -> Self {
            if name.is_empty() {
                panic!("Tên nhánh không được để trống.");
            }
            info!("Creating new Branch with name: {}", name);
            Branch {
                name,
                head: Arc::new(Mutex::new(head)),
                commits: Arc::new(Mutex::new(Vec::new())),
                archived: false,
                last_active,
            }
        }

        pub fn archive_if_inactive(&mut self, threshold: u64) {
            if self.last_active < threshold {
                info!("Archiving branch: {} due to inactivity.", self.name);
                self.archived = true;
            }
        }

        pub fn add_commit(&self, commit: Commit) {
            let mut commits = self.commits.lock().unwrap();
            info!("Adding commit with hash: {} to branch: {}", commit.hash, self.name);
            commits.push(commit.clone());

            let mut head = self.head.lock().unwrap();
            *head = commit;
        }

        pub fn get_name(&self) -> &String {
            info!("Getting name for Branch: {}", self.name);
            &self.name
        }

        pub fn get_head(&self) -> Arc<Mutex<Commit>> {
            info!("Getting head for Branch: {}", self.name);
            Arc::clone(&self.head)
        }

        pub fn get_commits(&self) -> Arc<Mutex<Vec<Commit>>> {
            info!("Getting commits for Branch: {}", self.name);
            Arc::clone(&self.commits)
        }

        pub fn is_archived(&self) -> bool {
            info!("Checking if Branch: {} is archived", self.name);
            self.archived
        }

        pub fn get_last_active(&self) -> u64 {
            info!("Getting last active timestamp for Branch: {}", self.name);
            self.last_active
        }
    }

    pub struct Remote {
        name: String,
        url: String,
        branches: Arc<Mutex<Vec<Branch>>>,
    }

    impl Remote {
        pub fn new(name: String, url: String) -> Self {
            if name.is_empty() || url.is_empty() {
                panic!("Tên và URL của remote không được để trống.");
            }
            info!("Creating new Remote with name: {} and url: {}", name, url);
            Remote {
                name,
                url,
                branches: Arc::new(Mutex::new(Vec::new())),
            }
        }

        pub fn archive_old_branches(&self, limit: usize) {
            let mut branches = self.branches.lock().unwrap();
            info!("Archiving old branches for Remote: {}, limit: {}", self.name, limit);
            if branches.len() > limit {
                branches.truncate(limit);
            }
        }

        pub fn add_branch(&self, branch: Branch) {
            let mut branches = self.branches.lock().unwrap();
            info!("Adding branch: {} to Remote: {}", branch.name, self.name);
            branches.push(branch);
        }

        pub fn get_name(&self) -> &String {
            info!("Getting name for Remote: {}", self.name);
            &self.name
        }

        pub fn get_url(&self) -> &String {
            info!("Getting URL for Remote: {}", self.name);
            &self.url
        }

        pub fn get_branches(&self) -> Arc<Mutex<Vec<Branch>>> {
            info!("Getting branches for Remote: {}", self.name);
            Arc::clone(&self.branches)
        }
    }

    pub struct File {
        path: String,
        blob: Arc<Mutex<Blob>>,
        metadata: Arc<Mutex<Metadata>>,
    }

    impl File {
        pub fn new(path: String, blob: Blob, metadata: Metadata) -> Self {
            if path.is_empty() {
                panic!("Đường dẫn tệp không được để trống.");
            }
            info!("Creating new File with path: {}", path);
            File {
                path,
                blob: Arc::new(Mutex::new(blob)),
                metadata: Arc::new(Mutex::new(metadata)),
            }
        }

        pub fn get_path(&self) -> &String {
            info!("Getting path for File: {}", self.path);
            &self.path
        }

        pub fn get_blob(&self) -> Arc<Mutex<Blob>> {
            info!("Getting blob for File: {}", self.path);
            Arc::clone(&self.blob)
        }

        pub fn get_metadata(&self) -> Arc<Mutex<Metadata>> {
            info!("Getting metadata for File: {}", self.path);
            Arc::clone(&self.metadata)
        }
    }

    pub struct Folder {
        path: String,
        files: Arc<Mutex<Vec<File>>>,
        folders: Arc<Mutex<Vec<Folder>>>,
    }

    impl Folder {
        pub fn new(path: String) -> Self {
            if path.is_empty() {
                panic!("Đường dẫn thư mục không được để trống.");
            }
            info!("Creating new Folder with path: {}", path);
            Folder {
                path,
                files: Arc::new(Mutex::new(Vec::new())),
                folders: Arc::new(Mutex::new(Vec::new())),
            }
        }

        pub fn get_path(&self) -> &String {
            info!("Getting path for Folder: {}", self.path);
            &self.path
        }

        pub fn get_files(&self) -> Arc<Mutex<Vec<File>>> {
            info!("Getting files for Folder: {}", self.path);
            Arc::clone(&self.files)
        }

        pub fn get_folders(&self) -> Arc<Mutex<Vec<Folder>>> {
            info!("Getting folders for Folder: {}", self.path);
            Arc::clone(&self.folders)
        }

        pub fn add_file(&self, file: File) {
            let mut files = self.files.lock().unwrap();
            info!("Adding File: {} to Folder: {}", file.path, self.path);
            files.push(file);
        }

        pub fn add_folder(&self, folder: Folder) {
            let mut folders = self.folders.lock().unwrap();
            info!("Adding Folder: {} to Folder: {}", folder.path, self.path);
            folders.push(folder);
        }
    }

    pub struct Tag {
        name: String,
        commit: Arc<Mutex<Commit>>,
        message: String,
    }

    impl Tag {
        pub fn new(name: String, commit: Commit, message: String) -> Self {
            if name.is_empty() || message.is_empty() {
                panic!("Tên tag và thông điệp không được để trống.");
            }
            info!("Creating new Tag with name: {}", name);
            Tag {
                name,
                commit: Arc::new(Mutex::new(commit)),
                message,
            }
        }

        pub fn get_name(&self) -> &String {
            info!("Getting name for Tag: {}", self.name);
            &self.name
        }

        pub fn get_commit(&self) -> Arc<Mutex<Commit>> {
            info!("Getting commit for Tag: {}", self.name);
            Arc::clone(&self.commit)
        }

        pub fn get_message(&self) -> &String {
            info!("Getting message for Tag: {}", self.name);
            &self.message
        }
    }

    pub struct Stash {
        id: String,
        changes: Arc<Mutex<Vec<change::Summary>>>,
        message: String,
        timestamp: u64,
    }

    impl Stash {
        pub fn new(id: String, message: String, timestamp: u64) -> Self {
            if id.is_empty() || message.is_empty() {
                panic!("ID và thông điệp không được để trống.");
            }
            info!("Creating new Stash with ID: {}", id);
            Stash {
                id,
                changes: Arc::new(Mutex::new(Vec::new())),
                message,
                timestamp,
            }
        }

        pub fn add_change(&self, change: change::Summary) {
            let mut changes = self.changes.lock().unwrap();
            info!("Adding change to Stash with ID: {}", self.id);
            changes.push(change);
        }

        pub fn get_id(&self) -> &String {
            info!("Getting ID for Stash: {}", self.id);
            &self.id
        }

        pub fn get_changes(&self) -> Arc<Mutex<Vec<change::Summary>>> {
            info!("Getting changes for Stash with ID: {}", self.id);
            Arc::clone(&self.changes)
        }

        pub fn get_message(&self) -> &String {
            info!("Getting message for Stash with ID: {}", self.id);
            &self.message
        }

        pub fn get_timestamp(&self) -> u64 {
            info!("Getting timestamp for Stash with ID: {}", self.id);
            self.timestamp
        }
    }

    pub struct Hook {
        name: String,
        event: String,
        script: String,
    }

    impl Hook {
        pub fn new(name: String, event: String, script: String) -> Self {
            if name.is_empty() || event.is_empty() || script.is_empty() {
                panic!("Tên, sự kiện và script không được để trống.");
            }
            info!("Creating new Hook with name: {} for event: {}", name, event);
            Hook {
                name,
                event,
                script,
            }
        }

        pub fn get_name(&self) -> &String {
            info!("Getting name for Hook: {}", self.name);
            &self.name
        }

        pub fn get_event(&self) -> &String {
            info!("Getting event for Hook: {}", self.name);
            &self.event
        }

        pub fn get_script(&self) -> &String {
            info!("Getting script for Hook: {}", self.name);
            &self.script
        }
    }
}
