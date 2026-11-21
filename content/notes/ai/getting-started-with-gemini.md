---
title: "My Daily Learning Workflow with Antigravity CLI"
date: "2026-11-21"
---

## Why Automate the Daily Habit

Writing a blog post from scratch every evening creates friction. Instead, I use **Antigravity CLI** with a repo-local skill (`.agents/skills/daily-learn/SKILL.md`) to turn raw learning dumps into clean Markdown notes:

1. Dump rough notes, snippets, or questions from what I studied today.
2. Answer 1–2 quick Feynman-style check-in questions to close knowledge gaps.
3. Save the structured Markdown file into the right folder under `content/`.
4. Commit and push to GitHub to trigger an automatic **GCP Cloud Run** deploy.

## Key Takeaway

Keeping the blog architecture radically simple — just folders and Markdown files — lets me spend 95% of my energy actually learning and building AI projects.
