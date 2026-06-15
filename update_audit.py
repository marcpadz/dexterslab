import re

with open("porting-specs/desktop_adaptation_audit.md", "r") as f:
    content = f.read()

# Replace Sidebar section 6.3 and 6.4 if present
new_63 = """| 6.3 | Project Context | **KEEP** | None (page) | Projects are isolated session contexts. A Project groups system instructions and reference files, which are applied to any chat thread assigned to it. They appear in the `Chat` tab sub-navigation. |
| 6.4 | Multi-Tab Sidebar | **ADAPT** | Layout (Sidebar) | Replace generic single-list sidebar with a 5-tab system (Chat, Agent, Notes, Work, Playground) where each tab renders context-specific sub-navigation items. |
| 6.5 | Knowledge Graph / Memories | **ADAPT** | Layout | Decouple Knowledge Graph from Projects. It is now accessed via the global `Memories` surface under the `Agent` tab. |"""

content = re.sub(r'\|\s*6\.3\s*\|\s*Project View.*?\|.*?(?=\n\|)', new_63, content, flags=re.DOTALL)

with open("porting-specs/desktop_adaptation_audit.md", "w") as f:
    f.write(content)
print("Updated desktop_adaptation_audit.md")
