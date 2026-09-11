---
name: visual-explainer
description: Use when an authoritative Markdown design exists and needs its HTML companion, before the user approves the written design, to generate a standalone accessible and responsive design.html.
license: MIT
metadata:
  artifact-layout: docs/superplanner
  output: standalone HTML
---

# Visual Explainer

Generate `design.html` beside an existing authoritative `design.md`. The HTML explains the same design visually; it does not introduce requirements or replace the Markdown. If they disagree, update the HTML to match the Markdown.

Generate and present the companion before requesting approval of the written design. Approval of an earlier in-chat design may authorize writing the artifacts, but written-design approval is not a prerequisite for this skill.

## Read And Select

Read the complete design before writing. Extract its purpose, scope, actors, components, interfaces, flows, state, decisions, constraints, alternatives, failure behavior, risks, testing, and rollout. Give every important design decision an appropriate visual representation:

| Information | Preferred representation |
| --- | --- |
| Components and boundaries | Architecture map with labeled edges |
| Request, event, or handoff order | Sequence or swimlane diagram |
| Lifecycle and recovery | State-transition diagram |
| Data entities and ownership | Relationship map or compact schema table |
| Alternatives and tradeoffs | Side-by-side comparison matrix |
| User interface behavior | Annotated wireframe at realistic proportions |
| Constraints, risks, and invariants | Structured callouts tied to affected elements |

Do not force all content into a flowchart. Combine representations only when each answers a distinct question. Use concise supporting prose rather than copying the entire Markdown.

## Build A Standalone Blueprint

Write one complete HTML document with semantic HTML, inline CSS, and only minimal inline JavaScript when interaction materially improves comprehension. Do not depend on CDNs, web fonts, frameworks, remote images, external templates, build steps, or files other than the companion `design.md`. Prefer CSS and inline SVG for diagrams.

The result should look like an engineering blueprint rather than a generic dashboard:

- a clear title, initiative context, source-of-truth notice, and an
  approval-independent design-stage label such as `Proposed Design`; never mirror
  mutable approval metadata into HTML;
- a consistent visual grammar for systems, actors, stores, external services, and boundaries;
- labeled connectors with direction and payload or responsibility where relevant;
- stable section navigation and a deliberate reading order;
- enough detail to inspect interfaces and failure paths without decorative clutter;
- restrained color, high information density, and print-friendly styling;
- responsive layout with no horizontal page overflow at phone, tablet, and desktop widths.

Use CSS variables, fluid sizing, wrapping grids, `max-width`, and targeted media queries. Make wide diagrams responsive or place them in clearly labeled, keyboard-accessible scroll regions. Preserve legibility at 320 CSS pixels and when printed.

## Accessibility

- Use landmarks, heading hierarchy, lists, tables, and a skip link.
- Ensure keyboard access and visible focus for every interactive control.
- Meet WCAG AA contrast and never encode meaning by color alone.
- Give each informative SVG a `<title>` and `<desc>` and connect them with ARIA; mark decorative graphics hidden.
- Provide text equivalents for diagrams and useful alternative text for images.
- Respect `prefers-reduced-motion`; avoid autoplay, flashing, and unnecessary animation.
- Keep body text readable and touch targets large enough on mobile.

## Integrity Review

Cross-check every label, relationship, sequence, constraint, and
approval-independent stage label against substantive `design.md` content. Do not
compare or copy the approval record into HTML. Remove visual claims not supported
by the design. Make omissions explicit only when the Markdown explicitly marks
them unresolved. Preview at mobile and desktop widths and inspect print output
when practical.

## Output Checklist

- `design.html` is beside and faithful to `design.md`.
- The file opens directly without a server, network, package install, or missing asset.
- The selected diagrams fit the information rather than a preset template.
- Architecture boundaries, interface directions, main flow, and failure paths are legible.
- Important decisions, exclusions, risks, and test strategy are represented.
- Semantic structure, keyboard operation, contrast, text alternatives, and reduced motion are covered.
- Phone, tablet, desktop, and print layouts avoid clipping and unreadable text.
- No placeholder, broken link, unsupported claim, or external dependency remains.

After the checklist passes, open the finished local `design.html` for the user whenever the active harness has browser capability. If no browser capability exists, report the exact file path instead.
