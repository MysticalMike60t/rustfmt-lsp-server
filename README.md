<div align="center">
<h1 align="center">
Configurations Language Server
</h1>
<p align="center">
<i>Syntax highlighting</i>, <i>autocompletion</i>, and more for various different configuration files!
</p>
<p align="center">
<span align="center">
<span>Developed for use with </span>
<span><b><a href="https://github.com/MysticalMike60t/zed-rustfmt">rustfmt</a></b> (more coming soon)</span>
<span> extension for </span>
<span><b><a href="https://github.com/zed-industries/zed">Zed</a></b></span>
<span>.</span>
</span>
</p>
</div>

<div align="center">
<img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/MysticalMike60t/configs-lsp-server/release.yml?style=plastic">
<img alt="Deps.rs Repository Dependencies" src="https://img.shields.io/deps-rs/repo/github/MysticalMike60t/configs-lsp-server?style=plastic">
</div>

<div align="center">
<img alt="GitHub License" src="https://img.shields.io/github/license/MysticalMike60t/configs-lsp-server?style=plastic">
<img alt="GitHub commits since latest release" src="https://img.shields.io/github/commits-since/MysticalMike60t/configs-lsp-server/latest?sort=semver&style=plastic">
<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/MysticalMike60t/configs-lsp-server?style=plastic">
<img alt="GitHub repo file or directory count" src="https://img.shields.io/github/directory-file-count/MysticalMike60t/configs-lsp-server?style=plastic">
<img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/MysticalMike60t/configs-lsp-server?style=plastic">
</div>

<div>
<h2>Current supported files</h2>
<p>Last updated on <b>16/04/2026</b></p>
<table>

</table>
<ol>
<li><code>rustfmt.toml</code> <sup id="fb1"><a href="#f1">1</a></sup></li>
<li><code>.rustfmt.toml</code> <sup id="fb1"><a href="#f1">1</a></sup></li>
</ol>
</div>

<div>
<h2>Why does this exist?</h2>
<p>
I created this because I love having snippets, autocomplete, highlighting, etc.
</p>
<p>
I am working on creating more of these language servers for different types
of configuration files in the future.
</p>
</div>

<div>
<h2>More Info</h2>
<p>
Any <b>Python</b> files (<code>*.py[w][c]</code>, <code>.python-version</code>, <code>pyproject.toml</code>, <code>.venv/</code>, etc) are not
needed for the extension. These are <strong>just</strong> for <strong>usage during development</strong><sup id="fb2"><a href="#f2">2</a></sup>.
</p>
<p>
This also includes <i>all</i> files inside of the <code>.dev/</code> folder.
</p>
</div>

<hr />

<div id="document-footnote">
<ol>
<li name="footnote-1" id="f1">
<p name="footnote-1-content">
<span><a href="#fb1">↩</a> <strong>Resources I used for development</strong></span>
<ul>
<li><span><a href="https://rust-lang.github.io/rustfmt/"><b>Rustfmt</b> documentation</a></span></li>
<li><span><a href="https://zed.dev/docs/extensions/"><b>Zed</b> extension documentation</a></span></li>
<li><span><a href="https://github.com/zed-industries/zed/"><b>Zed</b> source-code</a></span></li>
</ul>
</p>
</li>
<li name="footnote-2" id="f2">
<p name="footnote-2-content">
<span><a href="#fb2">↩</a> <b>Development specific usage</b>:</span>
<br />
<br />
<span data-type-sentence><strong>Code, folders, etc</strong> — made specifically for <strong>developers</strong>, and <i>NOT</i> end-users.</span>
<span data-type-sentence>All of these files can be deleted, and the <b>language server</b> will work as expected.</span>
<span data-type-sentence>If it <strong>does</strong> make an impact on the main functionality, please <a href="https://github.com/MysticalMike60t/configs-lsp-server/issues/new/choose">Open an Issue</a>.</span>
<span data-type-sentence><span><b>Currently tracked development files/folders</b></span><span>presented in <code>.gitignore</code> format.</span></span>
<ul>
<li><code>.zed/</code></li>
<li><code>.venv/</code><span> </span><sup>File(s) not currently used</sup></li>
<li><code>.python-version</code><span> </span><sup>File(s) not currently used</sup></li>
<li><code>pyproject.toml</code><span> </span><sup>File(s) not currently used</sup></li>
<li><code>*.spec</code><span> </span><sup>File(s) not currently used</sup></li>
<li><code>*.sh</code><span> </span><sup>File(s) not currently used</sup></li>
<li><code>*.bat</code><span> </span><sup>File(s) not currently used</sup></li>
<li><code>*.cmd</code><span> </span><sup>File(s) not currently used</sup></li>
<li><code>*.ps1</code><span> </span><sup>File(s) not currently used</sup></li>
</ul>
</p>
</li>
</ol>
</div>
