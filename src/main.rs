use serde::{Deserialize, Serialize};
use std::error::Error;

pub trait Checkable
where
    Self: Sized,
{
    fn is_accurate(&self) -> bool;
    fn is_accurate_with_sources(&self, trusted: Vec<Self>) -> Result<bool, String>;
    fn accuracy_score(&self) -> u32;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Source {
    location: String,
    trusted: bool,
}

impl Checkable for Source {
    fn is_accurate(&self) -> bool {
        return self.trusted;
    }

    fn is_accurate_with_sources(&self, trusted: Vec<Self>) -> Result<bool, String> {
        todo!();
    }

    fn accuracy_score(&self) -> u32 {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Statistic {
    sources: Vec<Source>,
    description: String,
    value: i32,
}

impl PartialEq for Statistic {
    fn eq(&self, other: &Self) -> bool {
        if self.description == other.description && self.value == other.value {
            return true;
        }

        let mut count = 0;
        for i in self.description.split_whitespace() {
            if other.description.contains(i) {
                count += 1;
            }
        }

        let av_len_majority =
            0.55 * ((self.description.len() + other.description.len()) / 2) as f32;

        if count > av_len_majority.round() as u32 {
            if self.value == other.value {
                return true;
            }
        } else {
            for i in &self.sources {
                if !i.trusted {
                    return false;
                }
            }
        }

        return false;
    }
}

impl Checkable for Statistic {
    fn is_accurate(&self) -> bool {
        for i in &self.sources {
            if !i.is_accurate() {
                return false;
            }
        }

        return true;
    }

    fn is_accurate_with_sources(&self, trusted: Vec<Self>) -> Result<bool, String> {
        for i in trusted {
            if !(*self == i) {
                return Ok(false);
            }
        }

        return Ok(true);
    }

    fn accuracy_score(&self) -> u32 {
        todo!();
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Information {
    website_name: String,
    is_trusted: bool,
    website_topics: Vec<String>,
    pub statistics: Vec<Statistic>,
}

impl Checkable for Information {
    fn is_accurate(&self) -> bool {
        if self.is_trusted {
            return true;
        }

        for i in &self.statistics {
            if !i.is_accurate() {
                return false;
            }
        }

        return true;
    }

    fn is_accurate_with_sources(&self, trusted: Vec<Self>) -> Result<bool, String> {
        if self.is_trusted {
            return Ok(true);
        }

        let mut hasdone = false;

        for j in &trusted {
            let mut relevant = false;
            for i in &self.website_topics {
                if j.website_topics.contains(i) {
                    relevant = true;
                    hasdone = true;
                    break;
                }
            }

            if relevant {
                for i in &self.statistics {
                    if !i.is_accurate_with_sources(j.statistics.clone()).unwrap() {
                        return Ok(false);
                    }
                }
            }
        }

        if !hasdone {
            return Err("Error no relevant sources".to_string());
        }

        return Ok(true);
    }

    fn accuracy_score(&self) -> u32 {
        todo!();
    }
}

const GOV_FILE: &'static str = include_str!("example.json");
const FALSE_WEBSITEFILE: &'static str = include_str!("falsewebsite.json");
const FB_POST_FILE: &'static str = include_str!("facebookpost.json");

fn main() -> Result<(), Box<dyn Error>> {
    let gov_info: Information = serde_json::from_str(GOV_FILE)?;
    let falsewebsite_info: Information = serde_json::from_str(FALSE_WEBSITEFILE)?;
    let facebook_info: Information = serde_json::from_str(FB_POST_FILE)?;

    if falsewebsite_info.is_accurate_with_sources(vec![gov_info.clone()])? {
        println!("The website {} is accurate", falsewebsite_info.website_name);
    } else {
        println!(
            "The website {} is not accurate",
            falsewebsite_info.website_name
        );
    }

    if facebook_info.is_accurate_with_sources(vec![gov_info.clone()])? {
        println!("The website {} is accurate\n", facebook_info.website_name);
    } else {
        println!("The website {} is not accurate", facebook_info.website_name);
    }

    Ok(())
}
