import os.path
import os.path
import subprocess

import click
import requests
from lxml import etree

session = requests.session()


class Problem(object):
    def __init__(self, name, samples):
        self.name = name
        self.samples = samples


template = """//! %s

use std::io::{stdin, stdout};
use std::io::prelude::*;
use codefoces::scanner::Scanner;


fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
    macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
    let t: usize = scanner.token();
    for _ in 0..t {
        puts!("Ok");
    }
}

fn main() {
    solve(Scanner::new(stdin().lock()), &mut stdout().lock())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            %s
        ];
        let functions: Vec<fn(Scanner<_>, &mut _)> = vec![
            solve
        ];
        for func in functions {
            for tc in testcases.chunks(2) {
                input! {Scanner::new(tc[0].as_bytes()), a: [i32]}
                let mut output = Vec::new();
                func(Scanner::new(tc[0].as_bytes()), &mut output);
                assert_eq!(String::from_utf8(output).unwrap(), tc[1], "input: {}", tc[0]);
            }
        }
    }
}
"""


def generate_file(filename, problem: Problem):
    lines = []

    def format_sample(text):
        return text.replace('\r', '').replace('\n', r'\n')

    for ip, op in problem.samples:
        lines.append(f'"{format_sample(ip)}",')
        lines.append(f'"{format_sample(op)}",')
    with open(filename, 'w', encoding="utf-8") as f:
        content = template % (problem.name, '\n'.join(lines))
        f.write(content)


class Atcoder(object):
    contest_url = 'https://atcoder.jp/contests/'

    def get_problem(self, contest, gid):
        url = f"https://atcoder.jp/contests/{contest}/tasks/{contest}_{gid}"
        response = session.get(url)
        html = etree.HTML(response.text)
        name = html.xpath("//span[@class='h2']/text()")[0].strip()
        sample_input = html.xpath("//h3[contains(text(), 'Sample Input')]/../pre/text()")
        sample_output = [i.text or '' for i in html.xpath("//h3[contains(text(), 'Sample Output')]/../pre")]
        return Problem(name, list(zip(sample_input, sample_output)))

    def contest_gid_list(self, contest):
        url = f'{self.contest_url}{contest}'
        response = session.get(url)
        html = etree.HTML(response.text)
        gid_list = html.xpath("//th[text() = 'Task']/../../../tbody//tr/td[1]/text()")
        return gid_list

    def handle(self, url: str):
        problems = []
        if url.startswith(self.contest_url):
            contest, _, gid = url.removeprefix(self.contest_url).partition('/tasks/')
            if gid == '':
                for gid in self.contest_gid_list(contest):
                    problems.append((contest, gid))
            else:
                problems.append((contest, gid))
        for contest, gid in problems:
            filename = f'codeforces_{contest}_{gid}'.lower()
            filepath = f"src/bin/{filename}.rs"
            if os.path.exists(filepath):
                print(f"{filepath} exist!")
                return
            samples = self.get_problem(contest, gid)
            generate_file(filepath, samples)
            print(filepath)
            subprocess.run(f'git add {filepath}', shell=True)


class Codeforces(object):
    contest_url = 'https://codeforces.com/contest/'
    problemset_url = 'https://codeforces.com/problemset/problem/'

    def get_problem(self, contest, gid):
        url = f"{self.problemset_url}{contest}/{gid}"
        response = session.get(url)
        html = etree.HTML(response.text)
        if response.text.count('class="test-example-line'):
            sample_input = ['\n'.join(html.xpath("//div[@class='input']/pre/div/text()"))]
        else:
            sample_input = [i.lstrip() for i in html.xpath("//div[@class='input']/pre/text()")]
        name = html.xpath("//div[@class='title']/text()")[0].strip()
        sample_output = [i.lstrip() for i in html.xpath("//div[@class='output']/pre/text()")]
        return Problem(name, list(zip(sample_input, sample_output)))

    def contest_gid_list(self, contest):
        url = f"https://codeforces.com/contest/{contest}"
        response = session.get(url)
        html = etree.HTML(response.text)
        urls = html.xpath(f"//a[contains(@href, '/contest/{contest}/problem/')]/@href")
        return [i.partition('/problem/')[2] for i in set(urls)]

    def handle(self, url: str):
        problems = []
        if url.startswith(self.contest_url):
            contest, _, gid = url.removeprefix(self.contest_url).partition('/problem/')
            if gid == '':
                for gid in self.contest_gid_list(contest):
                    problems.append((contest, gid))
            else:
                problems.append((contest, gid))
        elif url.startswith(self.problemset_url):
            contest, _, gid = url.removeprefix(self.problemset_url).partition('/')
            problems.append((contest, gid))
        else:
            raise Exception(f"unknown codeforces url {url}")
        for contest, gid in problems:
            filename = f'codeforces_{contest}_{gid}'.lower()
            filepath = f"src/bin/{filename}.rs"
            if os.path.exists(filepath):
                print(f"{filepath} exist!")
                return
            samples = self.get_problem(contest, gid)
            generate_file(filepath, samples)
            print(filepath)
            subprocess.run(f'git add {filepath}', shell=True)


@click.command()
@click.argument("url_list", nargs=-1)
def cli(url_list):
    for url in url_list:
        if url.startswith('https://codeforces.com/'):
            Codeforces().handle(url)
        elif url.startswith('https://atcoder.jp/'):
            Atcoder().handle(url)
        else:
            raise Exception(f"unknown url {url}")


if __name__ == '__main__':
    cli()
