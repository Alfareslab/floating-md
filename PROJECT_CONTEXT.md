# 🧠 Project Context: Floating MD (The Desktop Bridge)

> **⚠️ GOVERNANCE:** This project adheres to the `MASTER_CONSTITUTION.md`. Refer to it for architectural rules, roles, and coding standards.
> **📅 إنشاء:** 2026-01-23  
> **👤 المطور:** أحمد صالح  
> **📍 الموقع:** طنطا، مصر  
> **الإصدار الحالي:** v0.4.0 (Polish & Advanced Features)
> **📅 آخر تحديث:** 2026-02-07

---

## 1. Project Vision & Architecture

> **🎯 Goal:** شريط أدوات عائم لنظام Windows يعمل كوسيط بين تطبيقات AI Chat وبيئة العمل المحلية. يوفر تحكم كامل في النسخ/اللصق ومعالجة Markdown والعربية باستخدام الماوس فقط.

> **🏗️ Architecture:** **Clean Architecture** - فصل كامل بين Rust Backend (OS Integration) و React Frontend (UI Logic) و SQLite (Data Layer)

### 1.1 Core Components

#### 1️⃣ **Rust Backend (System Layer)**
- **Tech:** Rust 2021 Edition + windows-rs 0.58+
- **Role:** 
  - التحكم في Window (WS_EX_NOACTIVATE لمنع Focus Stealing)
  - مراقبة الحافظة (WM_CLIPBOARDUPDATE)
  - إرسال اختصارات الكيبورد (SendInput API)
  - Docking Logic و Multi-Monitor Support

#### 2️⃣ **React Frontend (UI Layer)**
- **Tech:** React 18 + TypeScript + Tailwind CSS
- **Role:**
  - عرض Dock و Hover Bubbles
  - محرر Markdown مع معاينة
  - Smart Scrub Interface (Sparkle ✨)
  - RTL/BiDi Support

#### 3️⃣ **SQLite Database (Data Layer)**
- **Tech:** SQLite + FTS5 Extension
- **Role:**
  - حفظ سجل الحافظة
  - بحث فوري في النصوص العربية
  - تخزين العناصر المثبتة (Pinned Items)
  - حفظ تكوينات النافذة

#### 4️⃣ **unified + Shiki (Markdown Engine)**
- **Tech:** unified + remark + rehype + Shiki
- **Role:**
  - تحويل Markdown إلى HTML مع Syntax Highlighting
  - دعم BiDi (عزل الأكواد من النص العربي)
  - Custom theme "Midnight Candy"

### 1.2 Security & Data

- **Auth:** لا يوجد - التطبيق محلي بالكامل
- **Data Storage:** SQLite محلي (لا إرسال بيانات للسحابة)
- **Sensitive Data:** لا توجد - كل البيانات على جهاز المستخدم
- **Clipboard Privacy:** البيانات المنسوخة لا تُشارك مع أي طرف ثالث

---

## 2. Current Feature Set (Status)

> ✅ = مكتمل | 🔄 = قيد التنفيذ | ⏸️ = مؤجل

### 2.1 Backend Features (Rust)

#### ✅ **Window Module** (Phase 2 Complete)
- ✅ `noactivate.rs`: WS_EX_NOACTIVATE + Window Subclassing Fallback
- ✅ `docking.rs`: Multi-Monitor Support + Edge Snapping (Smooth Drag Optimized 🏎️)

#### ✅ **Input Module** (Phase 2 Complete)
- ✅ `clipboard.rs`: Read/Write + Content Type Detection
- ✅ `keyboard.rs`: SendInput API + Ctrl+C/V Injection

#### ✅ **Database Module** (Phase 2 Complete)
- ✅ `schema.rs`: SQLite + FTS5 Virtual Table + Triggers
- ✅ `queries.rs`: CRUD Operations + FTS5 Search

### 2.2 UI & UX (v0.4.0 Enhanced)

- 🔄 **Theme:** Dark Mode (Manus AI Inspired) - Tailwind configured
  - ألوان: Deep Charcoal (#1C1D21) + Cyan Accent (#88C0D0)
- ✅ **Languages:** English + Arabic (RTL Support Optimized)
- ✅ **Key Interface Elements:**
  - Floating Dock (80px collapsed, 300px expanded)
  - Hover Bubbles (تظهر بعد 200ms)
  - Markdown Editor Panel (Toolbar + AI Magic ✨)
  - Toast Notifications

### 2.3 Core Features (v0.4.0 Enhanced)

#### ✅ **Global Copy/Paste**
- Backend ready ✅ | UI ready ✅

#### ✅ **Clipboard History Manager**
- Backend ready ✅ | UI Fixed (Smart Positioning 🐛) ✅

#### ✅ **Markdown Editor**
- Formatting Toolbar ✅
- AI Magic Button (Summarize/Fix/Translate) ✅
- RTL Support ✅

#### ⏸️ **Smart Scrub**
- Logic pending | UI pending

#### ✅ **Window Management**
- Backend ready ✅ | Smooth Dragging ✅

---

## 3. DevOps & Scripts

> **ملاحظة:** سيتم إنشاء السكربتات في مجلد `/scripts` حسب `MASTER_CONSTITUTION`

- **`run_app.bat`**: تشغيل النسخة التطويرية وإغلاق النسخ السابقة ✅
- **`kill_app.bat`**: إجبار إغلاق التطبيق ✅
- **`build`**: `pnpm tauri build` (إنشاء MSI installer)

---

## 4. Known Issues & Resolutions (History)

> **المشروع جديد - لا توجد مشاكل محلولة بعد**

### القضايا المتوقعة:

#### ⚠️ **WS_EX_NOACTIVATE قد يفشل**
- **السبب:** بعض إصدارات Windows لا تدعمه بشكل كامل
- **الحل المخطط:** Fallback لحفظ واستعادة النافذة النشطة

#### ⚠️ **WebView2 قد لا يكون مثبت**
- **السبب:** Windows 10 القديم قد لا يحتوي على WebView2
- **الحل المخطط:** تضمين WebView2 Bootstrapper في Installer

#### ⚠️ **Multi-Monitor قد يسبب مشاكل**
- **السبب:** حفظ Absolute Coordinates يفشل عند تغيير ترتيب الشاشات
- **الحل المخطط:** حفظ Monitor ID + Relative Position

#### ✅ **Clipboard History Popup Positioning** (v0.4.0 Resolved)
- **المشكلة:** النافذة كانت تخرج عن الشاشة.
- **الحل:** Smart Positioning Logic + Viewport Clamping.

#### ✅ **Window Dragging Jitter** (v0.4.0 Resolved)
- **المشكلة:** تقطيع أثناء سحب النافذة.
- **الحل:** Debounced Snapping in Rust + Event-Driven Frontend.

---

## 5. Next Steps (Roadmap)

### ✅ المرحلة 1: Setup (v0.1.0) - COMPLETE
- ✅ Git Init + Initial Commit
- ✅ Tauri + React + TypeScript Project
- ✅ Tailwind CSS + PostCSS Configuration
- ✅ Core Dependencies (Zustand, unified, Shiki)
- **تم بواسطة:** Gemini 3 Flash

### ✅ المرحلة 2: Rust Backend (v0.2.0) - COMPLETE
- ✅ Window Module (noactivate + docking)
- ✅ Input Module (clipboard + keyboard)
- ✅ Database Module (schema + queries + FTS5)
- ✅ Tauri Commands Integration
- **تم بواسطة:** Gemini 3 Flash + Claude Opus 4.5 (fixes)

### ✅ المرحلة 3: Comprehensive Audit & Fixes (v0.3.0) - COMPLETE
- ✅ **P1 Fixes:** Focus Guardian, Buttons, Quit, Window Dragging
- ✅ **P2 Features:** Preview Mode, Load/Copy HTML
- ✅ **Infrastructure:** unified pipeline, toast notifications
- **تم بواسطة:** Claude Opus 4.5 (Thinking) + Claude Sonnet 4.5

### ✅ المرحلة 4: Polish & Advanced Features (v0.4.0) - COMPLETE
- ✅ **Clipboard History Fix:** Smart Positioning
- ✅ **Smooth Movement:** Debounced Snapping
- ✅ **Markdown Editor:** Toolbar + AI Magic ✨
- **تم بواسطة:** Claude Sonnet 4.5 (Thinking) + Claude Opus 4.5

### 🔄 المرحلة 5: Release Candidate (v0.5.0) - NEXT
- 🔄 **Smart Scrub Logic:** تنفيذ منطق تنظيف النصوص
- 🔄 **Installer & Distribution:** إعداد ملفات التثبيت
- 🔄 **Final QA:** اختبار شامل
- **النموذج المقترح:** Claude Sonnet 4.5

---

## 6. Technical Stack (Final Decision)

| الطبقة | التقنية المعتمدة | البديل | السبب |
|--------|------------------|--------|-------|
| **App Framework** | Tauri 2.0 | Electron ❌ | حجم أصغر (8MB vs 150MB) واستهلاك أقل |
| **Backend** | Rust 2024 | C++ ❌ | Memory Safety + Win32 Access |
| **Frontend** | React 18 | SolidJS ⏸️ | نظام بيئي أكبر (قرار نهائي معلق) |
| **State** | Zustand | Redux ❌ | أبسط وأخف (2.9 KB) |
| **Styling** | Tailwind 4.x | CSS Modules ❌ | إجماع كامل من التقارير |
| **Markdown** | unified + Shiki | Marked ❌ | مرونة أكبر وجودة أعلى |
| **Database** | SQLite + FTS5 | IndexedDB ❌ | بحث عربي أقوى |
| **Build Tool** | Vite 6.x | Webpack ❌ | أسرع وأبسط |

---

## 7. Compliance with MASTER_CONSTITUTION

### ✅ البروتوكول الثلاثي:
- **المطور (أحمد):** صاحب الرؤية والقرار النهائي
- **المشرف (AI Supervisor):** سيراجع خطط التنفيذ
- **الوكيل (AI Agent):** ينفذ التعليمات التقنية

### ✅ فلسفة الكود المثالي:
- **فصل الطبقات:** Rust (Backend) ↔️ React (UI) ↔️ SQLite (Data)
- **صفر ديون تقنية:** لا حلول مؤقتة بدون توثيق
- **الاستدامة:** كود يعيش لسنوات

### ✅ الهيكلة القياسية:
```
floating-md/
├── docs/           ✅ التوثيق (MASTER_CONSTITUTION, DESIGN_SPEC, audit/)
├── plans/          ✅ خطط التنفيذ (active: 05, archive: 01-04)
├── scripts/        ✅ سكربتات التشغيل (run_app.bat, kill_app.bat)
├── Backups/        ✅ النسخ الاحتياطية
├── .git/           ✅ Git Repository (active)
├── src-tauri/      ✅ Rust Backend (complete)
└── src/            ✅ React Frontend (all checks passed)
```

### ✅ دعم العربية الأصيل:
- RTL Support كامل
- Bidirectional Text Isolation
- خطوط IBM Plex Sans Arabic
- FTS5 مع Arabic Tokenization

---

## 8. AI Models Assignment (حسب SOTA_Models_2026.md)

| المرحلة | النموذج الموصى به | الحالة |
|---------|-------------------|--------|
| **المرحلة 1: Setup** | Gemini 3 Flash | ✅ مكتمل |
| **المرحلة 2: Rust Backend** | Claude Opus 4.5 (Thinking) | ✅ مكتمل |
| **المرحلة 3: Audit & Fixes** | **Claude Opus + Sonnet** | ✅ مكتمل |
| **المرحلة 4: Polish & Advanced** | Claude Sonnet 4.5 (Thinking) | ✅ مكتمل |

---

## 9. Performance Targets

| المقياس | الهدف | الملاحظات |
|---------|-------|-----------|
| **Cold Startup** | < 500ms | من الضغط حتى الظهور |
| **Memory (Idle)** | < 80 MB | في حالة الراحة |
| **Bundle Size** | < 15 MB | MSI Installer كامل |
| **FTS Search** | < 10ms | على 10,000 عنصر |
| **Focus Stealing** | 0% | ✅ تحقق (Focus Guardian) |
| **Smart Scrub** | > 95% | دقة حذف المقدمات |
| **Smooth Drag** | 60 FPS | ✅ تحقق (Debounce) |

---

## 10. Design Reference

> **ملف التصميم:** `docs/DESIGN_SPEC.md`

يحتوي على:
- Color System (Manus Aesthetic)
- Typography Stack (Geist + IBM Plex Arabic)
- Component Layouts (Hover Bubble)
- Smart Scrub UX Flow
- Focus State Feedback
- CSS Variables Reference

---

**آخر تحديث:** 2026-02-07  
**الإصدار:** v0.4.0 (Polish Complete)  
**الحالة:** **Ready for Release Candidate (v0.5.0)**
