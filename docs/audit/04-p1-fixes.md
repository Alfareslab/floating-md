# P1 Critical Fixes Report

> **تاريخ:** 2026-02-06  
> **المرحلة:** 4 من خطة `04-comprehensive-audit-plan.md`  
> **النموذج:** Claude Opus 4.5 (Thinking)

---

## ✅ الإصلاحات المكتملة

### 1. 🔘 إصلاح: الأزرار لا تستجيب

**المشكلة:**  
أزرار الـ Toolbar (Copy, Paste, Cut, Delete, Select All) لا تعمل عند الضغط عليها.

**التشخيص:**  
- الـ clicks كانت تصل لـ JavaScript بشكل صحيح (تم التأكد بـ `alert()`)
- المشكلة الحقيقية كانت في Focus Guardian

**الحل:**  
إصلاح Focus Guardian (انظر أدناه) أدى لتشغيل الأزرار.

**الملفات المعدلة:**  
- لا يوجد تعديل مباشر (المشكلة كانت في noactivate)

---

### 2. 🛡️ إصلاح: Focus Guardian Stack Overflow

**المشكلة:**  
التطبيق يقفل فوراً مع خطأ `thread 'main' has overflowed its stack`

**التشخيص:**  
```
[INFO] Subclassed window HWND(0x7207f2) for WM_MOUSEACTIVATE interception
[INFO] Applied WS_EX_NOACTIVATE... (مرة ثانية!)
[INFO] Subclassed window... (مرة ثانية!)
thread 'main' has overflowed its stack ❌
```

الـ `subclass_window_for_noactivate` كان يُستدعى مرتين:
1. المرة الأولى: يخزّن الـ original wndproc
2. المرة الثانية: يخزّن الـ **new wndproc** كـ "original" → infinite loop

**الحل:**  
إضافة `IS_SUBCLASSED` AtomicBool guard flag:

```rust
/// Flag to prevent double subclassing (which causes stack overflow)
static IS_SUBCLASSED: AtomicBool = AtomicBool::new(false);

pub fn subclass_window_for_noactivate(hwnd: isize) -> Result<(), String> {
    // Guard: Prevent double subclassing
    if IS_SUBCLASSED.swap(true, Ordering::SeqCst) {
        log::warn!("Window already subclassed, skipping...");
        return Ok(());
    }
    // ... rest of function
}
```

**الملفات المعدلة:**  
- `src-tauri/src/window/noactivate.rs` (lines 8, 22, 60-65)

---

### 3. ❌ إصلاح: زر Quit لا يقفل التطبيق

**المشكلة:**  
زر الـ X (Quit) لا يقفل التطبيق مع خطأ:
```
Failed to close: window.close not allowed. 
Permissions: core:window:allow-close
```

**التشخيص:**  
Tauri 2.0 يتطلب permissions صريحة لعمليات النافذة.

**الحل:**  
إضافة permission في `capabilities/default.json`:

```json
{
  "permissions": [
    "core:default",
    "opener:default",
    "core:window:allow-close"  // ← تمت الإضافة
  ]
}
```

**الملفات المعدلة:**  
- `src-tauri/capabilities/default.json`

---

## 📋 ملخص التغييرات

| الملف | التعديل |
|-------|---------|
| `noactivate.rs` | إضافة guard flag لمنع double subclassing |
| `default.json` | إضافة `core:window:allow-close` permission |
| `ToolbarConfig.tsx` | إضافة error logging لزر Quit (للتشخيص) |

---

## ✅ نتائج الاختبار

| الوظيفة | الحالة |
|---------|--------|
| Copy | ✅ يعمل |
| Paste | ✅ يعمل |
| Cut | ✅ يعمل |
| Delete | ✅ يعمل |
| Select All | ✅ يعمل |
| Quit | ✅ يعمل |
| Focus Guardian | ✅ يعمل (Notepad يحتفظ بالـ Focus) |

---

## ⏭️ الخطوة التالية

المرحلة 5: تفعيل الوظائف الناقصة (P2)
- Preview Toggle
- Load from Clipboard  
- Copy HTML
- أزرار أخرى

**النموذج المسؤول:** Claude Sonnet 4.5
