# Al-Oikos PRD to Development Plan

## 📘 Project Overview
**Al-Oikos** (Arabic “This”, Greek “House”) is a one-stop platform for the global debating community, focused on British Parliamentary (BP) but expandable to other formats. It integrates tournament management, community building, video/audio chat, AI-assisted training, and debate analytics in one cohesive platform.

---

## 📅 MVP Timeline (Tentative)
| Milestone | Target Date |
|----------|--------------|
| Requirements Finalization | Week 1 |
| Design Phase | Week 2 |
| Core Architecture Setup | Week 3–4 |
| Core Modules Implementation (MVP) | Week 5–10 |
| Alpha Testing (Closed Houses) | Week 11 |
| Public Beta Launch | Week 13 |

---

## 🧱 Core Modules Breakdown

### 1. Authentication & User Management
- [ ] Signup/Login with email or OAuth
- [ ] Email/phone verification
- [ ] Role-based permissions (Admin, House Leader, Judge, Speaker, Guest)

### 2. Forums (General + House Specific)
- [ ] Threaded discussions
- [ ] Media/file sharing
- [ ] Moderation and reporting tools
- [ ] Notifications for mentions/replies

### 3. Communication (Real-time)
- [ ] In-app chat (1:1 and group)
- [ ] Audio Rooms (WebRTC)
- [ ] Video Conferencing (WebRTC)
- [ ] Moderator controls

### 4. Houses (Debate Clubs/Orgs)
- [ ] Create/manage House
- [ ] Dedicated spaces (forums, chats, calendar)
- [ ] Internal tournament handling

### 5. Tournaments & Tabbing
> See dedicated section below for Tabbing System Design
- [ ] Tournament creation (public/private)
- [ ] Registration with eligibility checks
- [ ] Team management
- [ ] Judge pool + conflicts
- [ ] Auto-draw generation (power-pairing, fold, etc.)
- [ ] Live results input
- [ ] Leaderboard updates

### 6. AI Integration
- [ ] Motion Generation (theme-based + random)
- [ ] AI Debaters (Sparring mode)
- [ ] AI Judges (Speech analysis + feedback)
- [ ] Open Adjudication Mode (score vs benchmark)

### 7. Reports & Moderation
- [ ] Ticket submission (bug, complaint, equity)
- [ ] Admin resolution panel
- [ ] Ban/suspend accounts

### 8. Admin Panel
- [ ] Monitor users and activity
- [ ] View system health
- [ ] Manage events/tickets
- [ ] Global announcements

### 9. Financial Services
- [ ] Stripe/Paystack integration
- [ ] Tournament entry fees
- [ ] House subscriptions
- [ ] Prize pool payouts

### 10. Event Calendar
- [ ] Global events list
- [ ] Filter by region or format
- [ ] Sync to House pages

### 11. Notifications
- [ ] Push notifications
- [ ] SMS/email integration
- [ ] In-app alerts and reminders

---

## 🧮 Dedicated Module: Tabbing System

### Objective
Design a robust, real-time tabbing system that retains all strengths of **Tabbycat** while fitting into Al-Oikos’ microservices and user experience flow.

### Features
#### Core Draw Logic
- Power pairing
- Randomized pairings
- Fold draws for BP
- Pull-up/pull-down logic
- Bye detection and AI-team pairing

#### Speaker & Team Ranking
- Margin calculation
- Team points and speaker scores
- Round-weighted averaging for best speakers
- Custom speaker awards (ESL, novice, etc.)

#### Judge Allocation & Conflicts
- Conflict resolution matrix (self-reported + inferred)
- Strike and clash system
- Chair and panel allocation
- Judge feedback forms (with anonymized results)

#### Real-Time Dashboard
- Visual round draw
- Drag-and-drop judge reassignment
- Auto-refresh round updates

#### Integration with Al-Oikos
- Pull House member/team info
- Send round info to video/audio rooms
- Auto-notify participants of room/time/motion
- Sync results into leaderboard module
- Allow AI judges to fill in ballots post-spar

#### AI Assist in Tabbing
- Suggest optimal panel based on past feedback
- Flag probable conflicts via name similarity detection
- Draft pre-tab motions based on event theme

#### Extensibility
- Support for other formats (AP, WSDC, etc.)
- Export draws and results to CSV/PDF
- Public round link for observers

---

### 🔧 Architecture Stack

| Layer | Tech |
|-------|------|
| Frontend | NextJS (React) |
| Backend | Rust (auth/core), Python (AI), C++ (RTC/video perf) |
| Protocols | REST, gRPC, WebSockets, Kafka |
| DB | PostgreSQL |
| Cache | Redis |
| App Shell | Flutter + PWA |
| Hosting | VPS or container orchestration (K8s/Docker Swarm) |

---

## 🚀 Suggested Development Phases

### Phase 1: MVP (Weeks 1–10)
- [ ] Authentication, Houses, Forums
- [ ] Video chat + in-app messaging
- [ ] Tournament creation + static tabbing (no live updates)
- [ ] Motion generation (manual/seeded themes)
- [ ] AI sparring (scripted GPT + text)

### Phase 2: Beta Expansion (Weeks 11–13)
- [ ] Real-time tabbing with judge management
- [ ] AI-generated motions + speeches
- [ ] AI feedback and open adjudication
- [ ] Financial services for events

### Phase 3: Scale + Gamify
- [ ] Leaderboards and badges
- [ ] Event calendar + notification system
- [ ] Analytics dashboard for debaters

---

## 👥 Team Structure (Suggested Roles)
- Product Lead / PM
- Backend Engineer (Rust/Python)
- AI Engineer (NLP + prompt tuning)
- Frontend Engineer (React/Next.js)
- DevOps (CI/CD, deployment)
- UX Designer
- Debate Expert (consultant)
