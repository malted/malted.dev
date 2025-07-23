#import "@preview/basic-resume:0.2.8": *

// Put your personal information here, replacing mine
#let name = "Ben Dixon"

#let email = "malted@malted.dev"
#let location = "$LOCATION$"
#let github = "github.com/malted"
#let linkedin = "linkedin.com/in/ma1ted"
#let phone = "+1 (415) 418-0612"
#let personal-site = "malted.dev"

#show link: set underline(offset: 2pt)
#show raw: set text(font: "IBM Plex Mono")

#show: resume.with(
  author: name,
  email: email,
  location: location,
  github: github,
  linkedin: linkedin,
  phone: phone,
  personal-site: personal-site,
  accent-color: "#1e3a8a",
  font: "IBM Plex Serif",
  font-size: 11.4pt,
  paper: "us-letter",
  author-position: left,
  personal-info-position: left,
)

/*
* Lines that start with == are formatted into section headings
* You can use the specific formatting functions if needed
* The following formatting functions are listed below
* #edu(dates: "", degree: "", gpa: "", institution: "", location: "", consistent: false)
* #work(company: "", dates: "", location: "", title: "")
* #project(dates: "", name: "", role: "", url: "")
* certificates(name: "", issuer: "", url: "", date: "")
* #extracurriculars(activity: "", dates: "")
* There are also the following generic functions that don't apply any formatting
* #generic-two-by-two(top-left: "", top-right: "", bottom-left: "", bottom-right: "")
* #generic-one-by-two(left: "", right: "")
*/

== Skills
*Languages:* Rust, TypeScript/JavaScript, Ruby, C\#, Swift, Python, GLSL \
*Frameworks:* Ruby on Rails, Next.js, Actix, SwiftUI, Rocket.rs, Unity, Vapor \
*Infrastructure:* Kubernetes, Docker,  PostgreSQL, SQLite, Cloudflare, Redis, Stripe

== Work Experience

#work(
  title: "Hack Club",
  location: "Vermont, USA",
  company: "Engineer",
  dates: "April 2023 – July 2025",
)
- Owned reliability and feature velocity across a suite serving 50k+ teenage hackers.
- *#link("https://highseas.hackclub.com")[High Seas]:* (Next.js) Online hackathon where 20k students logged 18.5 years of coding time in 3 months in exchange for \$350k+ in prizes; responsible for platform engineering, site reliability, and infrastructure. Announced at GitHub Universe 2024.
- *#link("https://github.com/hackclub/ai")[ai.hackclub.com]:* (Rust, Actix, Groq) Created a free OpenAI-schema-compatible #raw("/chat/completions") service; \~0.5B tokens processed since Jan 2025.
- *#link("https://hackclub.com/fiscal-sponsorship")[HCB (Hack Club Bank)]:* (Ruby on Rails, Stripe) Added fine-grained permissions, spending controls, check reminders, new-IP login alerts, export formats; supported bank-backend migration & Stripe webhook ledgering, hundreds of bug fixes + UX improvements for an open source neobank for 501(c)(3) fiscal sponsorship.
- *#link("https://github.com/hackclub/replit-lifeboat")[Replit Lifeboat]*: (Rust, K8s, S3) Reverse engineered Replit internals to build the only tool converting Replit history to timed Git commits; 1.09M files across 82k projects exported. Built in a \~weekend.
- Spun up *#link("https://ip.hackclub.com")[ip.hackclub.com]* (Bun) & *#link("https://ships.hackclub.com")[ships.hackclub.com]* (Three.js + GLSL) in \<72h each for events and congressional demos.
- *#link("https://summer.hackclub.com")[Summer of Making]:* (Ruby on Rails) Responsible for the platform engineering of the in-progress successor to High Seas (above). Built and launched in \~3 weeks with 15k users. Responsible for platform engineering and feature integration.

#work(
  title: "Unity Engineer",
  location: "England",
  company: "Freelance",
  dates: "March 2020 – Jan 2021",
)
- Delivered high-fidelity prototypes for indie studios, focussing on clean and performant code.
- Worked in large codebases to profile games using the Unity profiler suite and fix performance issues.

#work(
  title: "OSS Projects",
  company: "Personal"
)
- *malted.dev:* (Rust, Rocket, MapKit) Plaintext-streamed personal site #link("https://github.com/malted/malted.dev")[🔗]
- Node-based shader editor exporting Shadertoy-compatible GLSL. #link("https://github.com/hackclub/shadergraph")[🔗]
- Experimental SwiftUI app for temporal Git diff visualisation.#link("https://github.com/malted/gitgud")[🔗]
- First-person character controller for Three.js. #link("https://github.com/malted/charactercontroller")[🔗]

== Talks
- *RailsConf 2024* — "Raytracing In ActiveRecord"
- *MIT* — "How We Built The Largest Online Hackathon Ever"
- *Congressional App Challenge, US Capitol* — "How To Ship Real Projects With AI"
