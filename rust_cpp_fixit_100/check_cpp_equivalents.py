#!/usr/bin/env python3
from pathlib import Path
import re, subprocess, tempfile, sys
from concurrent.futures import ThreadPoolExecutor, as_completed
root=Path(__file__).resolve().parent
files=sorted(root.glob('*/solutions/*.rs'))
td_obj=tempfile.TemporaryDirectory(); td=Path(td_obj.name)
jobs=[]
for i,p in enumerate(files,1):
    text=p.read_text(encoding='utf-8')
    m=re.search(r'/\* CPP_EQUIVALENT_BEGIN\n(.*?)\nCPP_EQUIVALENT_END \*/',text,re.S)
    if not m:
        jobs.append((i,p,None)); continue
    cpp=td/f'{i:03d}.cpp'; cpp.write_text(m.group(1)+'\n',encoding='utf-8')
    jobs.append((i,p,cpp))
def check(job):
    i,p,cpp=job
    if cpp is None:return i,p,'missing C++ block'
    r=subprocess.run(['g++','-std=c++20','-Wall','-Wextra','-pedantic','-fsyntax-only',str(cpp)],capture_output=True,text=True)
    return i,p,None if r.returncode==0 else r.stderr
results=[]
with ThreadPoolExecutor(max_workers=12) as ex:
    futs=[ex.submit(check,j) for j in jobs]
    for f in as_completed(futs):results.append(f.result())
results.sort()
errors=[]
for i,p,e in results:
    if e: errors.append((p,e))
    else: print(f'[{i:03d}] OK {p.relative_to(root)}')
if errors:
    print('\nFAILURES:',file=sys.stderr)
    for p,e in errors: print(f'--- {p.relative_to(root)}\n{e}',file=sys.stderr)
    sys.exit(1)
print(f'Validated {len(files)} embedded C++ equivalents.')
