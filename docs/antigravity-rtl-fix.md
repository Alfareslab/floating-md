# Antigravity RTL Fix (Final V5)

# 🛠️ إصلاح اتجاه النصوص العربية (Antigravity RTL Fix)

> **المشكلة:** المحرر بيعاني من مشكلة في عرض النصوص العربية المخلوطة بالإنجليزي (Bi-di text)، حيث بيقلب ترتيب الفقرات ويخلي القراءة صعبة جداً.
>
> **الحل:** تعديل ملف الـ CSS الخاص بواجهة الشات في Antigravity عشان يدعم `unicode-bidi: plaintext` مع تحسينات للقوائم والأكواد.

---

## 📋 خطوات التفعيل

### 1️⃣ افتح ملف التنسيق
انسخ المسار ده وافتحه في المتصفح أو أي محرر نصوص عندك:

```
%LOCALAPPDATA%\Programs\Antigravity\resources\app\extensions\antigravity\cascade-panel.html
```

### 2️⃣ استبدل الكود
امسح محتوى الملف بالكامل واستبدله بالكود النهائي (V5) ده:

```html
<!doctype html>
<html dir="auto">

<head>
  <style>
    /* ===========================================
       Antigravity Arabic Fix v5 — Final
       هذا الملف مخصص للواجهة العربية
       للإنجليزي: استخدم الملف الافتراضي الأصلي
       =========================================== */

    /* ① اتجاه النص — الأساس */
    p, li, h1, h2, h3, h4, h5, h6, div, td, th {
      unicode-bidi: plaintext !important;
    }

    textarea, input, [contenteditable="true"] {
      unicode-bidi: plaintext !important;
    }

    pre, code, pre *, code * {
      direction: ltr !important;
      unicode-bidi: embed !important;
      text-align: left !important;
    }

    /* ② إصلاح القوائم — نقاط يمين + نص يمين */
    ul, ol {
      direction: rtl !important;
      text-align: right !important;
    }

    li {
      unicode-bidi: plaintext !important;
      text-align: right !important;
    }

    /* ③ الكود داخل السطر — شفاف */
    code:not(pre code) {
      background: transparent !important;
      border: none !important;
      padding: 1px 3px !important;
      border-radius: 3px !important;
    }

    /* ④ تباعد مريح */
    p {
      line-height: 1.8 !important;
      margin-bottom: 0.6em !important;
    }

    li {
      line-height: 1.7 !important;
      margin-bottom: 0.3em !important;
    }

    h1, h2, h3, h4, h5, h6 {
      line-height: 1.5 !important;
    }
  </style>
</head>

<body style="margin: 0">
  <div id="react-app" class="react-app-container" dir="auto"></div>
</body>

</html>
```

### 3️⃣ إعادة التشغيل
اقفل برنامج **Antigravity** وافتحه تاني.

---

## ✅ النتيجة (V5)
*   **اتجاه النص:** مظبوط تماماً مهما كان الخليط بين العربي والإنجليزي.
*   **القوائم (Lists):** النقاط والكلام محاذي لليمين (Right Aligned) لغلق الفجوة.
*   **الأكواد (Codes):** خلفية شفافة ونظيفة، واتجاه LTR سليم.
*   **القراءة:** مريحة للعين بفضل زيادة المسافات (Line Height).
