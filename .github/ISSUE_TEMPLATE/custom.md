---
name: Sprint Issue template
about: Standardise the impl of issues and discussion.
title: ''
labels: ''
assignees: ''

---

<!-- Everything in HTML comments will not render in filed issue, added only for documentation of template. Feel free to delete them or leave commented as is before submitting -->

<!-- Issue Tracker this action item belongs to. -->
- *this issue is included in the #<TRACKER ISSUE NUMBER> tracker for Sprint <NUMBER> milestone*
<!-- IF RELEVANT: Another filed issue that this action item also address, aside from tracker. -->
- *this issue formally addresses one prerequisite to the larger issue filed in #3*

---

<!-- Succinct description of the action item followed by check list of "acceptance criterion quality" sub-tasks that signal a successful implementation/solution -->
# summary
<!-- Example:

In order for the front-end to both *initiate* and communicate *back* to the rust module export, the Rust module needs to have a means to translate the following native game types to their JS equivalents:

  - [ ] `Board`
  - [ ] `GameState`
  
-->

<!-- Clarifying facts that do not belong in a succinct summary, e.g. points of confusion to clarify, issues in need of discussion in the comments or in next meeting, general concerns -->
## clarifications
<!-- Example:

- The rust module has `GameState`, but meeting minutes and the generated design outlines have this as `BoardState`. Which is it?
-->

<!-- A list of other specific action items already filed that may interfere with a complete implementation or otherwise complicate by adding further, unknown, requirements to this action item and *why* it poses such a threat. Otherwise, if the potential blocker is not actually filed, it ought to be done first and then return to this issue and edit to include it. Only when not suitable for an issue should it be listed w/o externally documenting. -->
##  potential blockers
<!-- Example:

- there is a great chance #23 will add to the criteria required by this issue, but there isn't much else to say until that time comes.

-->
