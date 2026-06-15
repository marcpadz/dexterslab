import re

with open("porting-specs/system_design.md", "r") as f:
    content = f.read()

# Add a section defining Projects as Session Contexts
projects_def = """### 4.7 Projects as Session Contexts
In v2, Projects are not strictly parent containers of chats. A Project is defined as an isolated contextual workspace containing custom instructions, linked files, and specific settings. Individual Chat Sessions or Threads are attached to a Project. This allows the agent to inject the Project's context into the active session without forcing the user to navigate a rigid hierarchical folder structure.
"""

if "### 4.7 Projects as Session Contexts" not in content:
    content = re.sub(r'(### 4.6 .*?\n.*?\n)(?=## 5\.)', r'\1\n' + projects_def + '\n', content, flags=re.DOTALL)

with open("porting-specs/system_design.md", "w") as f:
    f.write(content)
print("Updated system_design.md")
