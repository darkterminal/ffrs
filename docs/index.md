---
layout: default
title: Home
description: "Plain English media commands in, deterministic ffmpeg commands out."
---

<section class="hero">
  <div class="wrap hero-grid">
    <div>
      <p class="eyebrow">Plain English in · ffmpeg out</p>
      <h1>Say the conversion.<br>See the command.</h1>
      <p class="lede"><code>ffrs</code> translates readable media commands into <code>ffmpeg</code> commands. Use it as a CLI or as a Rust library. Inspect the exact command with <code>--dry-run</code> before anything runs.</p>
      <div class="cta-row">
        <a class="btn btn-primary" href="{{ '/manual/' | relative_url }}">Read the manual</a>
        <a class="btn" href="#install">Install with Cargo</a>
      </div>
    </div>
    <div class="terminal">
      <p class="terminal-label">Inspect before you run</p>
      <pre class="code"><span class="prompt">$ </span>ffrs --dry-run "convert video.mp4 to video.avi"
ffmpeg "-n" "-i" "video.mp4" "video.avi"</pre>
    </div>
  </div>
</section>

<section class="section">
  <div class="wrap">
    <h2>Three steps, no flag archaeology</h2>
    <p class="section-intro">You describe the outcome. <code>ffrs</code> validates it, prints the generated command, then executes it. The command is deterministic for a given input.</p>
    <ol class="steps">
      <li><span class="num">01</span><div><strong>Describe.</strong> Write <code>convert video.mp4 to video.avi</code> the way you would say it.</div></li>
      <li><span class="num">02</span><div><strong>Inspect.</strong> Add <code>--dry-run</code> to see the exact <code>ffmpeg</code> invocation. No <code>ffmpeg</code> install needed for this step.</div></li>
      <li><span class="num">03</span><div><strong>Run.</strong> Drop the flag to execute. Existing outputs are preserved by default; <code>--yes</code> opts into overwriting.</div></li>
    </ol>
  </div>
</section>

<section class="section">
  <div class="wrap">
    <h2>What it offers</h2>
    <p class="section-intro">A small tool with a narrow job: turn a few readable operations into correct command lines, and stay out of the way.</p>
    <dl class="feature-grid">
      <div class="feature"><dt>Convert &amp; transcode</dt><dd>Switch containers or choose a video and audio codec with one phrase.</dd></div>
      <div class="feature soft"><dt>Extract audio</dt><dd>Pull audio from a video without remembering stream maps or quality flags.</dd></div>
      <div class="feature"><dt>Resize</dt><dd>Target an explicit <code>WIDTHxHEIGHT</code>, or fall back to 1920x1080.</dd></div>
      <div class="feature soft"><dt>Dry run</dt><dd>Print the generated command first. Run it only when it looks right.</dd></div>
      <div class="feature"><dt>Safe defaults</dt><dd><code>-n</code> is passed by default, so an existing output file is never silently replaced.</dd></div>
      <div class="feature soft"><dt>Library API</dt><dd>The same tokenizer, parser, and builder are available from Rust.</dd></div>
    </dl>
  </div>
</section>

<section class="section" id="install">
  <div class="wrap split">
    <div>
      <h2 id="install">Install</h2>
      <p class="section-intro">Release archives are not published yet. Install from crates.io or build from source with a Rust 1.85+ toolchain.</p>
      <p>Actual conversion needs <code>ffmpeg</code> on your <code>PATH</code>. <code>--dry-run</code> does not.</p>
    </div>
    <div>
      <pre class="code">cargo install ffrs

# or, from a clone
git clone https://github.com/darkterminal/ffrs.git
cd ffrs
cargo install --path .

ffrs --version</pre>
    </div>
  </div>
</section>

<section class="section">
  <div class="wrap split">
    <div>
      <h2>Commands you can read</h2>
      <p class="section-intro">The pattern is <code>[operation] &lt;input&gt; to [parameter] &lt;output&gt;</code>. Quoted file names and absolute or relative paths work.</p>
      <p><a href="{{ '/manual/syntax/' | relative_url }}">See the full syntax reference →</a></p>
    </div>
    <div>
      <pre class="code">ffrs "convert clip.mp4 to clip.avi"
ffrs "extract audio from clip.mp4 to audio.mp3"
ffrs "resize clip.mp4 to 1280x720 small.mp4"
ffrs "transcode clip.mp4 to h265 out.mp4"
ffrs --interactive</pre>
    </div>
  </div>
</section>

<section class="section">
  <div class="wrap">
    <h2>Start with the manual</h2>
    <p class="section-intro">Installation details, every operation, validation rules, troubleshooting, and the full CLI reference are in the user manual. The <a href="https://docs.rs/ffrs">API reference</a> covers library usage.</p>
    <p><a class="btn btn-primary" href="{{ '/manual/' | relative_url }}">Open the user manual</a></p>
  </div>
</section>
