# 🧠 Project Context: Floating MD (The Desktop Bridge)

> **⚠️ GOVERNANCE:** This project adheres to the `MASTER_CONSTITUTION.md`. Refer to it for architectural rules, roles, and coding standards.
> **📅 إنشاء:** 2026-01-23  
> **👤 المطور:** أحمد صالح  
> **📍 الموقع:** طنطا، مصر  
> **الإصدار الحالي:** v0.1.0 (Planning Phase)

---

## 1. Project Vision & Architecture

> **🎯 Goal:** شريط أدوات عائم لنظام Windows يعمل كوسيط بين تطبيقات AI Chat وبيئة العمل المحلية. يوفر تحكم كامل في النسخ/اللصق ومعالجة Markdown والعربية باستخدام الماوس فقط.

> **🏗️ Architecture:** **Clean Architecture** - فصل كامل بين Rust Backend (OS Integration) و React Frontend (UI Logic) و SQLite (Data Layer)

### 1.1 Core Components

#### 1️⃣ **Rust Backend (System Layer)**
- **Tech:** Rust 2024 Edition + windows-rs 0.58+
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

### 2.1 UI & UX

- ⏸️ **Theme:** Dark Mode فقط (مستوحى من Manus AI)
  - ألوان: Deep Charcoal (#1C1D21) + Cyan Accent (#88C0D0)
- ⏸️ **Languages:** English + Arabic (RTL Support كامل)
- ⏸️ **Key Interface Elements:**
  - Floating Dock (80px collapsed, 300px expanded)
  - Hover Bubbles (تظهر بعد 200ms)
  - Markdown Editor Panel (قابل للتوسع)
  - Toast Notifications (Undo Support)

### 2.2 Core Features

#### ⏸️ **Global Copy/Paste**
- أزرار Copy/Paste ترسل Ctrl+C/V للنافذة النشطة
- حل مشكلة Focus Stealing باستخدام WS_EX_NOACTIVATE

#### ⏸️ **Clipboard History Manager**
- سجل لآخر 100 عنصر منسوخ
- تثبيت (Pin) للنصوص المتكررة
- بحث فوري باستخدام FTS5

#### ⏸️ **Markdown Editor**
- Edit Mode + Preview Mode
- زر "Load from Clipboard"
- RTL Support كامل
- Syntax Highlighting (Shiki)

#### ⏸️ **Smart Scrub (تنظيف نصوص AI)**
- اكتشاف مقدمات AI تلقائياً
- زر Sparkle ✨ يدوي
- Undo Toast (5 ثوان)

#### ⏸️ **Window Management**
- Always On Top
- Auto-Hide/Peeking
- Snap to Screen Edges
- Multi-Monitor Support
- حفظ الموضع عند إعادة التشغيل

---

## 3. DevOps & Scripts

> **ملاحظة:** سيتم إنشاء السكربتات في مجلد `/scripts` حسب `MASTER_CONSTITUTION`

- **`start_app`**: تشغيل النسخة المبنية (Production)
- **`dev_start`**: `pnpm tauri dev` (Development Mode)
- **`clean`**: حذف `target/` و `node_modules/`
- **`backup`**: إنشاء نسخة احتياطية في `Backups/`
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

---

## 5. Next Steps (Roadmap)

### المرحلة الحالية: **Planning & Setup** (v0.1.0)

1. ✅ **قراءة وتحليل المتطلبات**
2. ✅ **إنشاء خطة التنفيذ الأولية**
3. 🔄 **مراجعة القرارات التقنية مع المطور**
   - اختيار React vs SolidJS
   - تفعيل Window Vibrancy أم لا
   - Smart Scrub تلقائي أم يدوي
4. ⏸️ **إنشاء PROJECT_CONTEXT.md النهائي**
5. ⏸️ **البدء في المرحلة 1: Setup**

### المراحل القادمة:

- **v0.2.0:** Rust Backend (WS_EX_NOACTIVATE + Clipboard Monitor)
- **v0.3.0:** Design System (Tailwind + Fonts)
- **v0.4.0:** UI Components (Dock + Bubble + Editor)
- **v0.5.0:** Window Management (Docking + Multi-Monitor)
- **v0.6.0:** Testing & Optimization
- **v1.0.0:** Documentation & Release

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
├── docs/           ✅ التوثيق والخطط
├── scripts/        ⏸️ سكربتات الصيانة
├── Backups/        ⏸️ النسخ الاحتياطية
├── .git/           ⏸️ Git Repository
├── src-tauri/      ⏸️ Rust Backend
└── src/            ⏸️ React Frontend
```

### ✅ دعم العربية الأصيل:
- RTL Support كامل
- Bidirectional Text Isolation
- خطوط IBM Plex Sans Arabic
- FTS5 مع Arabic Tokenization

---

## 8. AI Models Assignment (حسب SOTA_Models_2026.md)

| المرحلة | النموذج الموصى به | السبب |
|---------|-------------------|-------|
| **المرحلة 1: Setup** | Gemini 3 Flash | مهام سريعة وبسيطة |
| **المرحلة 2: Rust Backend** | Claude Opus 4.5 (Thinking) | أصعب مرحلة - Win32 معقد |
| **المرحلة 3: Design System** | Claude Sonnet 4.5 (Thinking) | دقة التصميم |
| **المرحلة 4: UI Components** | Claude Sonnet 4.5 | جودة كود React |
| **المرحلة 5: Window Mgmt** | Claude Opus 4.5 (Thinking) | تعقيد Multi-Monitor |
| **المرحلة 6: Testing** | Claude Sonnet 4.5 | اختبار منهجي |
| **المرحلة 7: Docs** | Gemini 3 Flash | مهام نهائية سريعة |

---

## 9. Performance Targets

| المقياس | الهدف | الملاحظات |
|---------|-------|-----------|
| **Cold Startup** | < 500ms | من الضغط حتى الظهور |
| **Memory (Idle)** | < 80 MB | في حالة الراحة |
| **Bundle Size** | < 15 MB | MSI Installer كامل |
| **FTS Search** | < 10ms | على 10,000 عنصر |
| **Focus Stealing** | 0% | يجب ألا يحدث أبداً |
| **Smart Scrub** | > 95% | دقة حذف المقدمات |

---

**آخر تحديث:** 2026-01-23 00:40  
**الحالة:** **Planning - في انتظار القرارات التقنية النهائية**
