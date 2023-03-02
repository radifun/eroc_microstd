// =================================================================================================
// Copyright (c) 2023 Viet-Hoa Do <doviethoa@doviethoa.com>
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// =================================================================================================

use std::{fs, io, path};

// =================================================================================================
// Built-in convenient transformers
// =================================================================================================

pub fn read_file(p: &path::Path) -> io::BufReader<fs::File> {
    return io::BufReader::new(fs::File::open(p).unwrap());
}

pub fn write_file<T: Transformer>(mut inner: T, p: &path::Path) {
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    inner.write_to(&mut io::BufWriter::new(fs::File::create(p).unwrap()));
}

pub fn remove_stable_attr<T: Transformer>(inner: T) -> BlockRegex<T> {
    return BlockRegex::new(inner, None, r##"^\s*#!?\[stable\(.*"##, None, &[]);
}

pub fn remove_attr<T: Transformer>(inner: T, re: &str) -> BlockRegex<BlockRegex<T>> {
    let f = BlockRegex::new(inner, None, &format!(r##"^\s*#!?\[.*{}.*\].*"##, re), None, &[]);
    let f = BlockRegex::new(
        f,
        None,
        &format!(r##"^(\s*)#!?\[.*{}.*"##, re),
        Some(r##"^\)?\].*"##),
        &[],
    );

    return f;
}

pub fn remove_unstable_features<T: Transformer>(inner: T) -> BlockRegex<T> {
    return BlockRegex::new(
        inner,
        Some(r##"^\s*(?:///|#\[).*"##),
        r##"^(\s*)#\[unstable\(.*"##,
        Some(r##"^\}.*"##),
        &[],
    );
}

pub fn remove_doc_attr<T: Transformer>(inner: T) -> BlockRegex<T> {
    return BlockRegex::new(inner, None, r##"^\s*#!?\[doc\s*=.*"##, None, &[]);
}

pub fn remove_fn<T: Transformer>(inner: T, name: &str) -> BlockRegex<T> {
    return BlockRegex::new(
        inner,
        Some(r##"^\s*(?:///|#\[).*"##),
        format!(r##"^(\s*).*fn\s{}.*"##, name).as_str(),
        Some(r##"^\}.*"##),
        &[],
    );
}

pub fn remove_block<T: Transformer>(inner: T, name: &str) -> BlockRegex<T> {
    return BlockRegex::new(
        inner,
        Some(r##"^\s*(?:///|#\[).*"##),
        format!(r##"^(\s*){}.*"##, name).as_str(),
        Some(r##"^\}.*"##),
        &[],
    );
}

pub fn remove_line<T: Transformer>(inner: T, text: &str) -> BlockRegex<T> {
    return BlockRegex::new(inner, None, text, None, &[]);
}

pub fn remove_text<T: Transformer>(inner: T, text: &str) -> BlockRegex<T> {
    return BlockRegex::new(
        inner,
        None,
        &format!("^(.*){}(.*)", regex::escape(text)),
        None,
        &["${1}${2}"],
    );
}

// =================================================================================================
// Transformer
// =================================================================================================

/// Each transformer can process and produce one or more lines of text each time
/// [`Transformer::next_lines`] is called.
/// Multiple transformers can wrapped around each other to form a complete
/// text processing pipeline.
pub trait Transformer {
    /// Returns the next batch of text lines.
    ///
    /// If the end of file has been reached, return [`None`].
    ///
    /// If the next line cannot be produced but the end of file hasn't been reached,
    /// returns an empty [`Vec`].
    fn next_lines(&mut self) -> Option<Vec<String>>;

    /// Writes the final result to the specified [`Write`](std::io::Write) object.
    fn write_to<F: io::Write>(&mut self, f: &mut F) {
        loop {
            if let Some(lines) = self.next_lines() {
                for line in lines {
                    f.write(line.as_bytes()).unwrap();
                }
            } else {
                break;
            }
        }
    }
}

// Implement Transformer trait for Read ------------------------------------------------------------

impl<F: io::BufRead> Transformer for F {
    fn next_lines(&mut self) -> Option<Vec<String>> {
        let mut line = String::new();

        if let Ok(size) = self.read_line(&mut line) {
            if size > 0 {
                return Some(vec![line]);
            }
        }

        return None;
    }
}

// =================================================================================================
// Multiline search and replace using regular expression
// =================================================================================================

pub struct BlockRegex<T: Transformer> {
    inner: T,
    start_re: Option<regex::Regex>,
    commit_re: regex::Regex,
    end_re: Option<regex::Regex>,
    replace: Vec<String>,

    state: BlockRegexState,
    keep_lines: Vec<String>,
    prefix: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BlockRegexState {
    Ready,
    Started,
    Committed,
}

// Constructors ------------------------------------------------------------------------------------

impl<T: Transformer> BlockRegex<T> {
    pub fn new(
        inner: T,
        start_re: Option<&str>,
        commit_re: &str,
        end_re: Option<&str>,
        replace: &[&str],
    ) -> Self {
        return Self {
            inner,
            start_re: opt_str_to_regex(start_re),
            commit_re: regex::Regex::new(commit_re).unwrap(),
            end_re: opt_str_to_regex(end_re),
            replace: arr_str_to_vec_string(replace),

            state: BlockRegexState::Ready,
            keep_lines: Vec::<String>::new(),
            prefix: String::new(),
        };
    }
}

// Implement `Transformer` trait -------------------------------------------------------------------

impl<T: Transformer> Transformer for BlockRegex<T> {
    fn next_lines(&mut self) -> Option<Vec<String>> {
        let mut dst_lines = Vec::<String>::new();

        if let Some(src_lines) = self.inner.next_lines() {
            for line in &src_lines {
                match self.state {
                    BlockRegexState::Ready => {
                        let match_start = match_opt_regex(&self.start_re, line);
                        let match_commit = self.commit_re.captures(line);

                        if let Some(cap) = &match_commit {
                            for replace in &self.replace {
                                dst_lines.push(self.commit_re.replace(line, replace).to_string());
                            }

                            if self.end_re.is_some() {
                                self.state = BlockRegexState::Committed;
                                self.prefix = cap[1].to_string();
                            }
                        } else if match_start {
                            self.state = BlockRegexState::Started;
                            assert!(self.keep_lines.is_empty());
                            self.keep_lines.push(line.clone());
                        } else {
                            dst_lines.push(line.clone());
                        }
                    }

                    BlockRegexState::Started => {
                        let match_start = match_opt_regex(&self.start_re, line);
                        let match_commit = self.commit_re.captures(line);

                        if let Some(cap) = &match_commit {
                            if self.end_re.is_some() {
                                self.state = BlockRegexState::Committed;
                                self.prefix = cap[1].to_string();
                            } else {
                                self.state = BlockRegexState::Ready;
                            }

                            self.keep_lines.clear();

                            for replace in &self.replace {
                                dst_lines.push(self.commit_re.replace(line, replace).to_string());
                            }
                        } else if !match_start {
                            self.state = BlockRegexState::Ready;
                            dst_lines.append(&mut self.keep_lines);
                            dst_lines.push(line.clone());
                        } else {
                            self.keep_lines.push(line.clone());
                        }
                    }

                    BlockRegexState::Committed => {
                        if line.starts_with(&self.prefix) {
                            let truncated = &line[self.prefix.len()..];
                            let match_end = match_opt_regex(&self.end_re, truncated);

                            if match_end {
                                self.state = BlockRegexState::Ready;
                            }
                        }
                    }
                }
            }

            return Some(dst_lines);
        } else {
            if self.keep_lines.is_empty() {
                return None;
            } else {
                assert_eq!(self.state, BlockRegexState::Started);
                dst_lines.append(&mut self.keep_lines);
                return Some(dst_lines);
            }
        }
    }
}

// =================================================================================================
// Utilities
// =================================================================================================

fn opt_str_to_regex(value: Option<&str>) -> Option<regex::Regex> {
    if let Some(pattern) = value {
        return Some(regex::Regex::new(pattern).unwrap());
    } else {
        return None;
    }
}

fn arr_str_to_vec_string(value: &[&str]) -> Vec<String> {
    let mut v = Vec::<String>::new();

    for item in value.iter() {
        v.push(String::from(*item));
    }

    return v;
}

fn match_opt_regex(opt_pattern: &Option<regex::Regex>, text: &str) -> bool {
    if let Some(pattern) = opt_pattern {
        if pattern.is_match(text) {
            return true;
        }
    }

    return false;
}
