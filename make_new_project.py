#!/usr/bin/python3
import sys
import os
import shutil

def make_project_name():
    if len(sys.argv) > 2:
        return sys.argv[1], sys.argv[2]
    project = input('enter new project name:')
    if not project:
        print("please input invalid project name")
        sys.exit()
    author = input('enter new author:')
    if not author:
        print("please input invalid author")
        sys.exit()
    return project, author

def do_make_ifile(ofullname: str, project:str, author:str):
    with open(ofullname, mode='r', encoding='utf-8') as f:
        content = f.read()
    content = content.replace('nopname', project).replace('Nopname', project.capitalize())
    content = content.replace('NObodyGX', author).replace('nobodygx', author.lower())
    with open(ofullname, mode='w', encoding='utf-8') as f:
        f.write(content)

def travel_dir(r:str):
    files = []
    for entry in os.scandir(r):
        if entry.is_file():
            files.append(entry.path)
        elif entry.is_dir() and entry.name not in [".git", ".vscode", 'target', 'data_store']:
            files.extend(travel_dir(entry.path))
    return files

def do_make_project(project:str, author:str):
    print(project)
    print("ready to make project")
    idir = os.path.dirname(__file__)
    odir = os.path.join(os.path.dirname(os.path.dirname(__file__)), project)
    if not os.path.exists(odir):
        os.makedirs(odir)
    for ifullname in travel_dir(idir):
        if os.path.basename(ifullname) == os.path.basename(__file__):
            continue
        rpath = os.path.relpath(ifullname, idir)
        rpath = rpath.replace('nopname', project)
        ofullname = os.path.join(odir, rpath)
        print(ifullname, ofullname)
        if not os.path.exists(os.path.dirname(ofullname)):
            os.makedirs(os.path.dirname(ofullname))
        shutil.copy2(ifullname, ofullname)
        do_make_ifile(ofullname, project, author)


def main():
    project, author = make_project_name()
    do_make_project(project, author)

if __name__ == "__main__":
    main()
