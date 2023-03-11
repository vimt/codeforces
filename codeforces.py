import os.path
import os.path
import subprocess
from typing import List

import click
import requests
from lxml import etree

session = requests.session()
ua = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36'
session.headers['User-Agent'] = ua
if os.path.exists('cookie'):
    with open('cookie', 'r', encoding='utf-8') as f:
        cookie = f.read()
        session.headers['cookie'] = cookie.strip()


class Problem(object):
    def __init__(self, url, info: List[str], name, samples):
        self.url = url
        self.info = info
        self.name = name
        self.samples = samples


template = """//! %s
%s

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let a: Vec<i32> = (0..len).map(|_|scanner.next()).collect();
            puts!("Ok");
        };
        for _ in 0..t {
            go();
        }
    }
}

#[cfg(not(debug))]
fn main() {
    use codeforces::raw;
    let (stdin, mut stdout) = raw::in_out();
    my::solve(&mut Scanner::new(stdin), &mut stdout)
}

#[cfg(debug)]
fn main() {
    use codeforces::solves;
    use codeforces::tester::Tester;
    let t = Tester::new(solves!(my));
    %s
}
"""


def generate_file(filename, problem: Problem):
    lines = []

    def format_sample(text):
        return text.replace('\r', '').replace('\n', r'\n')

    for ip, op in problem.samples:
        lines.append(f't.test("{format_sample(ip)}",')
        lines.append(f'"{format_sample(op)}");')
    with open(filename, 'w', encoding="utf-8") as f:
        content = template % (problem.name, '\n'.join(f'//! {i}' for i in problem.info), '\n'.join(lines))
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
        return Problem(url, [], name, list(zip(sample_input, sample_output)))

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
                continue
            samples = self.get_problem(contest, gid)
            generate_file(filepath, samples)
            print(filepath)
            subprocess.run(f'git add {filepath}', shell=True)


class Codeforces(object):
    contest_url = 'https://codeforces.com/contest/'
    problemset_url = 'https://codeforces.com/problemset/problem/'
    status_url = 'https://codeforces.com/problemset/status/%s/problem/%s?order=BY_CONSUMED_TIME_ASC'
    luogu_url = 'https://www.luogu.com.cn/problem/solution/CF%s%s'

    def get_problem(self, contest, gid):
        url = f"{self.contest_url}{contest}/problem/{gid}"
        response = session.get(url)
        html = etree.HTML(response.text)
        sample_len = len(html.xpath("//div[@class='input']/pre"))
        if response.text.count('class="test-example-line'):
            sample_input = ['\n'.join(html.xpath("//div[@class='input']/pre/div/text()"))]
        else:
            sample_input = ['\n'.join(html.xpath(f"//div[@class='input'][{i}]/pre/text()")).strip() + '\n'
                            for i in range(1, sample_len + 1)]
        name = html.xpath("//div[@class='title']/text()")[0].strip()
        sample_output = ['\n'.join(html.xpath(f"//div[@class='output'][{i}]/pre/text()")).strip() + '\n'
                         for i in range(1, sample_len + 1)]
        info = [self.status_url % (contest, gid), self.luogu_url % (contest, gid)]
        return Problem(url, info, name, list(zip(sample_input, sample_output)))

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
                continue
            problem = self.get_problem(contest, gid)
            generate_file(filepath, problem)
            print(filepath)
            subprocess.run(f'git add {filepath}', shell=True)


class LuoGu(object):
    problem_url = f'https://www.luogu.com.cn/problem/'
    solution_url = 'https://www.luogu.com.cn/problem/solution/'
    record_url = 'https://www.luogu.com.cn/record/list?status=&pid=%s&page=1&orderBy=1'

    def get_problem(self, pid):
        url = self.problem_url + pid
        response = session.get(url)
        html = etree.HTML(response.text)
        title = html.xpath('//h1/text()')[0]
        sample_input = html.xpath("//h3[contains(text(), '输入样例')]/following::pre[1]/code/text()")
        sample_output = html.xpath("//h3[contains(text(), '输出样例')]/following::pre[1]/code/text()")
        sample_output = [i.rstrip() + '\n' for i in sample_output]
        return Problem(url, [self.record_url % pid], title, list(zip(sample_input, sample_output)))

    def handle(self, url: str):
        problems = []
        if url.startswith(self.problem_url):
            pid = url.removeprefix(self.problem_url).strip()
            assert '/' not in pid
            problems.append(pid)
        for pid in problems:
            filename = f'luogu_{pid}'.lower()
            filepath = f'src/bin/{filename}.rs'
            if os.path.exists(filepath):
                print(f'{filepath} exist')
                continue
            problem = self.get_problem(pid)
            generate_file(filepath, problem)
            print(filepath)
            subprocess.run(f'git add {filepath}', shell=True)


class Nowcoder(object):
    contest_url = 'https://ac.nowcoder.com/acm/contest/'
    problem_url = 'https://ac.nowcoder.com/acm/problem/'

    def get_problem(self, url):
        response = session.get(url)
        html = etree.HTML(response.text)
        title = html.xpath("//div[@class='question-title']/text()")[1].strip()
        sample_input = html.xpath("//h2[text()='输入']/..//pre/text()")
        sample_output = html.xpath("//h2[text()='输出']/..//pre/text()")
        return Problem(url, [url], title, list(zip(sample_input, sample_output)))

    def contest_gid_list(self, contest):
        url = f"https://ac.nowcoder.com/acm/contest/problem-list?id={contest}"
        response = session.get(url)
        data = response.json()['data']['data']
        return [i['index'] for i in data]

    def handle(self, url: str):
        problems = []
        if url.startswith(self.contest_url):
            contest, _, gid = url.removeprefix(self.contest_url).partition('/')
            if gid == '':
                for gid in self.contest_gid_list(contest):
                    problems.append((f'{contest}_{gid}', self.contest_url + f'{contest}/{gid}'))
            else:
                problems.append((f'{contest}_{gid}', self.contest_url + f'{contest}/{gid}'))
        elif url.startswith(self.problem_url):
            gid = url.removeprefix(self.problem_url)
            problems.append((gid, url))
        else:
            raise Exception(f"unknown codeforces url {url}")
        for name, url in problems:
            filename = f'nowcoder_{name}'.lower()
            filepath = f"src/bin/{filename}.rs"
            if os.path.exists(filepath):
                print(f"{filepath} exist!")
                continue
            problem = self.get_problem(url)
            generate_file(filepath, problem)
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
        elif url.startswith('https://www.luogu.com.cn/'):
            LuoGu().handle(url)
        elif url.startswith('https://ac.nowcoder.com/'):
            Nowcoder().handle(url)
        else:
            raise Exception(f"unknown url {url}")


if __name__ == '__main__':
    cli()
