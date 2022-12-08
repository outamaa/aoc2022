use std::str::FromStr;

type Tokens = Vec<Vec<String>>;

#[derive(Debug, PartialEq)]
enum DirEntry {
    File(usize, String),
    Directory(Vec<DirEntry>, String)
}

impl FromStr for DirEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cds = s
            .split("$ cd ")
            .map(|dir_str| {
                let mut dir_lines = dir_str
                    .lines()
                    .map(String::from)
                    .collect::<Vec<String>>();
                dir_lines.reverse();
                dir_lines
            })
            .collect::<Vec<Vec<String>>>();

        cds.reverse(); // Reverse for more convenient parsing via pop()
        cds.pop(); // Pop empty vec

        let (dir, cds) = parse_directory(cds)?;

        assert!(cds.is_empty());

        Ok(dir)
    }

}

impl DirEntry {
    fn directory_sizes(&self) -> Vec<usize> {
        match self {
            DirEntry::File(_, _) => vec![],
            DirEntry::Directory(entries, _) => {
                let mut recurred = entries
                    .iter()
                    .fold(vec![], |mut v, entry| match entry {
                        DirEntry::File(_, _) => v,
                        DirEntry::Directory(_, _) => {
                            let mut directory_sizes = entry.directory_sizes();
                            v.append(&mut directory_sizes);
                            v
                        }
                    });
                recurred.push(self.size());

                recurred
            }
        }
    }

    fn size(&self) -> usize {
        match self {
            DirEntry::File(size, _) => *size,
            DirEntry::Directory(entries, _) => entries
                .iter()
                .map(|entry| entry.size())
                .sum::<usize>()
        }
    }
}

/// Pop and return Ok on "cd ..", otherwise Err
fn parse_up(mut cds: Tokens) -> Result<Tokens, Tokens> {
    match cds.last() {
        None => Ok(cds),
        Some(entry) => match entry.last() {
            None => Err(cds),
            Some(dir_name) => match dir_name.as_str() {
                ".." => {
                    cds.pop();
                    Ok(cds)
                }
                _ => Err(cds)
            }
        }
    }

}

fn parse_directory(mut cds: Tokens) -> Result<(DirEntry, Tokens), String> {
    let _ = cds.last().ok_or_else(|| "end of input".to_string())?;
    let mut entry_list = cds.pop().unwrap();
    let dir_name = entry_list.pop().unwrap().clone();
    entry_list.pop().unwrap(); // drop the ls line
    let mut entries = Vec::with_capacity(entry_list.len());

    // Files
    for entry in entry_list {
        if !entry.starts_with("dir") {
            let mut file = entry.split(" ");
            entries.push(DirEntry::File(
                file.next().unwrap().parse().unwrap(),
                file.next().unwrap().to_string()
            ));
        }
    }

    // Recur directories, here be dragons
    loop {
        let up = parse_up(cds);
        let is_up = up.is_ok();
        cds = up.unwrap_or_else(|t| t);
        if is_up {
            break;
        }
        let (entry, new_cds) = parse_directory(cds)?;
        cds = new_cds;
        entries.push(entry);
    }
    Ok((DirEntry::Directory(entries, dir_name), cds))
}

// Star 1
fn sum_of_sizes_smaller_than(dir: &DirEntry, limit: usize) -> usize {
    dir
        .directory_sizes()
        .iter()
        .filter(|&&size| size <= limit)
        .sum()
}

// Star 2
fn smallest_size_to_delete(dir: &DirEntry) -> usize {
    let mut sizes = dir
        .directory_sizes();
    sizes.sort();

    let &used_space = sizes.last().unwrap();
    let space_to_free = used_space - 40000000;

    *sizes
        .iter()
        .find(|&&size| size >= space_to_free)
        .unwrap()
}


#[cfg(test)]
mod tests {
    use crate::day7::{DirEntry, smallest_size_to_delete, sum_of_sizes_smaller_than};

    #[test]
    fn test_example1() {
        let input = include_str!("example.txt");
        let dir = input.parse::<DirEntry>().unwrap();

        assert_eq!(
            sum_of_sizes_smaller_than(&dir, 100000),
            95437
        );
    }

    #[test]
    fn test_input1() {
        let input = include_str!("input.txt");

        let dir = input.parse::<DirEntry>().unwrap();

        assert_eq!(
            sum_of_sizes_smaller_than(&dir, 100000),
            1232307
        );
    }

    #[test]
    fn test_example2() {
        let input = include_str!("example.txt");
        let dir = input.parse::<DirEntry>().unwrap();

        assert_eq!(
            smallest_size_to_delete(&dir),
            24933642
        );
    }

    #[test]
    fn test_input2() {
        let input = include_str!("input.txt");

        let dir = input.parse::<DirEntry>().unwrap();

        assert_eq!(
            smallest_size_to_delete(&dir),
            7268994
        );
    }

}