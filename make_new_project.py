#!/usr/bin/python3
import sys
import os
import shutil
import enum


def test_case_converter():
    cc = CaseConverter("ABcEFg")
    print(cc.to_case(CaseConverter.Mode.Snake))
    print(cc.to_case(CaseConverter.Mode.Kebab))
    print(cc.to_case(CaseConverter.Mode.Camel))
    print(cc.to_case(CaseConverter.Mode.Pascal))
    print(cc.to_case(CaseConverter.Mode.Upper))

class CaseConverter(object):
    class Mode(enum.Enum):
        Snake = 0   # aa_bb
        Kebab = 1   # aa-bb
        Camel = 2   # aaBb
        Pascal = 3  # AaBb
        Upper = 4   # AA_BB
        Lower = 5   # aabb

    def __init__(self, name:str):
        self._init_data(name)

    def _split_data(self, name:str):
        data = []
        word = ""
        for x in name:
            if x.islower():
                word += x
                continue
            if word:
                data.append(word.lower())
            word = ""
            word += x
        data.append(word.lower())
        return data

    def _init_data(self, name:str):
        self.name = name
        self.data = []
        if name.count('-') > 0:
            self.data = name.split('-')
        elif name.count('_') > 0:
            self.data = name.split('_')
        elif name.count(" ") > 0:
            self.data = list(filter(lambda x: len(x) > 0, [x for x in name.split(' ')]))
        else:
            self.data = self._split_data(name)

    def to_case(self, mode, use_origin=False):
        if use_origin:
            return self.name
        if mode == CaseConverter.Mode.Snake:
            return "_".join(self.data)
        if mode == CaseConverter.Mode.Kebab:
            return "-".join(self.data)
        if mode == CaseConverter.Mode.Camel:
            return "".join([x.capitalize() if i != 0 else x for i, x in enumerate(self.data)])
        if mode == CaseConverter.Mode.Pascal:
            return "".join([x.capitalize() for x in self.data])
        if mode == CaseConverter.Mode.Upper:
            return "_".join([x.upper() for x in self.data])
        if mode == CaseConverter.Mode.Lower:
            return "".join(x.lower() for x in self.data)
        return self.name

class ProjectParam(object):
    def __init__(self, projcet:str="", author:str="", keep_author:bool=False):
        self.project = projcet
        self.author = author
        self.keep_author = keep_author

def make_project_param() -> ProjectParam:
    param = ProjectParam()

    if len(sys.argv) > 3:
        param.project = sys.argv[1]
        param.author = sys.argv[2]
        flag = str(sys.argv[3]).strip()
        if flag in("Y", "Yes", "yes", "True", "true", "1"):
            param.keep_author = True
        return param
    project = input('enter new project name:')

    if not project:
        print("please input invalid project name")
        sys.exit()
    param.project = project
    author = input('enter new author:')
    if not author:
        print("please input invalid author")
        sys.exit()
    param.author = author
    keep_author_case = input('whether keep author case: Y/N')
    if keep_author_case.strip() in("Y", "Yes", "yes", "True", "true", "1"):
        param.keep_author = True
    return param

def travel_dir(r:str):
    files = []
    for entry in os.scandir(r):
        if entry.is_file():
            files.append(entry.path)
        elif entry.is_dir() and entry.name not in [".git", 'target', 'data_store']:
            files.extend(travel_dir(entry.path))
    return files

def do_make_project():
    param = make_project_param()
    print(f"ready to make project: {param.project}")
    cc = CaseConverter(param.project)
    idir = os.path.dirname(__file__)
    odir = os.path.join(os.path.dirname(os.path.dirname(__file__)), cc.to_case(CaseConverter.Mode.Snake))
    if not os.path.exists(odir):
        os.makedirs(odir)
    for ifullname in travel_dir(idir):
        if os.path.basename(ifullname) == os.path.basename(__file__):
            continue
        rpath = os.path.relpath(ifullname, idir)
        rpath = rpath.replace('nop_name', cc.to_case(CaseConverter.Mode.Snake))
        ofullname = os.path.join(odir, rpath)
        print(ifullname, ofullname)
        if not os.path.exists(os.path.dirname(ofullname)):
            os.makedirs(os.path.dirname(ofullname))
        shutil.copy2(ifullname, ofullname)
        do_make_ifile(ofullname, param)
    do_make_readme(odir, param)

def do_make_ifile(ofullname: str, param: ProjectParam):
    with open(ofullname, mode='r', encoding='utf-8') as f:
        content = f.read()
    pcc = CaseConverter(param.project)
    acc = CaseConverter(param.author)
    content = content.replace('nop_name', pcc.to_case(CaseConverter.Mode.Snake))
    content = content.replace('nop-name', pcc.to_case(CaseConverter.Mode.Kebab))
    content = content.replace('NopName', pcc.to_case(CaseConverter.Mode.Pascal))
    content = content.replace('nopName', pcc.to_case(CaseConverter.Mode.Camel))
    content = content.replace('NOP_NAME', pcc.to_case(CaseConverter.Mode.Upper))
    if param.keep_author:
        content = content.replace('noa_name', acc.to_case(CaseConverter.Mode.Lower))
        content = content.replace('noa-name', acc.to_case(CaseConverter.Mode.Lower))
        content = content.replace('NoaName', acc.to_case(CaseConverter.Mode.Pascal))
        content = content.replace('noaName', acc.to_case(CaseConverter.Mode.Pascal))
    else:
        content = content.replace('noa_name', acc.to_case(CaseConverter.Mode.Snake))
        content = content.replace('noa-name', acc.to_case(CaseConverter.Mode.Kebab))
        content = content.replace('NoaName', acc.to_case(CaseConverter.Mode.Pascal))
        content = content.replace('noaName', acc.to_case(CaseConverter.Mode.Camel))
        content = content.replace('NOA_NAME', acc.to_case(CaseConverter.Mode.Upper))
    with open(ofullname, mode='w', encoding='utf-8') as f:
        f.write(content)

def do_make_readme(odir:str, param: ProjectParam):
    pcc = CaseConverter(param.project)
    with open(os.path.join(odir, 'README.md'), mode='w+', encoding='utf-8') as f:
        f.write("\n".join([
            f"# {pcc.to_case(CaseConverter.Mode.Pascal)}",
            "",
            "write your readme",
            ""
        ]))

def main():
    do_make_project()

if __name__ == "__main__":
    main()
