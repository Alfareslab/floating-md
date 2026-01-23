# **Floating MD: Phase 2 Visual Design & Feature Logic Specification**

## **1\. Visual System: The "Manus" Aesthetic Architecture & Color Theory**

The transition from a conceptual wireframe to a high-fidelity interface for "Floating MD" requires a rigorous adherence to the "Manus" aesthetic benchmark. This aesthetic is not merely a stylistic preference but a functional requirement for a tool intended to bridge the gap between high-intelligence AI agents and local desktop environments. The visual language of Manus—and by extension, the target for Floating MD—is defined by a philosophy of "dense yet breathable" minimalism. It rejects the stark, high-contrast binaries of early dark modes in favor of a sophisticated, layered approach that prioritizes long-duration legibility and cognitive clarity. This section deconstructs that aesthetic into actionable specifications for color and typography, with a specific focus on the complexities of Arabic (RTL) support.

### **1.1 The "Deep Charcoal" Paradigm: Beyond Pure Black**

A critical analysis of the Manus AI interface, alongside comparable high-end productivity tools (such as Linear, Vercel, and Raycast), reveals a consistent rejection of pure black (\#000000) for primary interface surfaces. While pure black is energy-efficient on OLED displays, it creates significant usability issues in desktop utility contexts. Pure black backgrounds cause "smearing" artifacts when scrolling text on high-contrast panels, and the extreme contrast ratio with white text can lead to halation (visual blooming) that fatigues the eye during extended coding or reading sessions.1  
Instead, the "Manus" look relies on a **Deep Charcoal** architecture. This palette utilizes cool, blue-tinted grays that recede visually while maintaining a sense of depth and "tech" sophistication. By elevating the background luminance slightly above zero, the interface minimizes eye strain and allows for the use of subtle borders and shadows to define hierarchy, rather than relying solely on jarring background color changes.  
The following color system is derived from a forensic analysis of modern AI agent interfaces, specifically targeting the Manus dark theme ecosystem.3 This palette is designed to function harmoniously within the Windows 11 "Mica" and "Acrylic" design language while maintaining a distinct, professional identity.

#### **1.1.1 Structural Color System**

The structural colors define the physical presence of the dock. Because Floating MD is a stationary, always-on-top utility, its background must be distinct enough to separate it from the user's wallpaper or underlying applications (like VS Code or Word) but subtle enough not to compete for attention.

| Token Name | Usage Context | Hex Code | Rationale & Theoretical Underpinning |
| :---- | :---- | :---- | :---- |
| **bg-primary** | **Main Dock Shell** | **\#1C1D21** | This specific hex code is identified as a highly accurate match for the foundational layer of modern AI agent interfaces. It is a near-black hue with a very subtle blue-purple undertone. This undertone is crucial; it prevents the interface from feeling like a "void" or a dead pixel area, giving it a subtle "alive" quality that integrates well with cool-toned desktop wallpapers.5 It provides a neutral stage for vibrant syntax highlighting without clashing. |
| **bg-surface** | **Cards, Panels, Inputs** | **\#2E3440** | This lighter, slate-tinted gray is used for elevated surfaces, such as the Clipboard History cards or the expanded Markdown editor. The shift from \#1C1D21 to \#2E3440 creates a perception of depth through luminance (lightness) rather than drop shadows, which keeps the interface looking crisp and modern. This color mimics the "Nord" palette's approach to surface layering, ensuring text remains legible without requiring high-contrast borders.4 |
| **border-subtle** | **Dividers, Card Borders** | **\#3E4451** | A low-contrast border color used to define edges. In dark mode, shadows are often invisible or muddy. Manus and similar tools rely on 1px borders to distinct "surfaces" from the "background." This color is light enough to be seen but dark enough to be ignored when focusing on content.3 |
| **bg-hover** | **Interactive Elements** | **\#3B4252** | This token represents a slightly lighter shift from bg-surface. It is essential for the "alive" feel of the dock. When a user hovers over a timestamp or icon, the background should shift instantly to this color, providing immediate, tactile feedback that the element is actionable. |
| **bg-active** | **Active/Focus State** | **\#15161A** | A slightly darker shade used for input fields or active editor areas to create a "recessed" look, simulating a writable surface that sits "inside" the dock.2 |

#### **1.1.2 Typography Colors: Hierarchy via Opacity**

In the Manus aesthetic, text hierarchy is established primarily through **opacity** and **weight** rather than by introducing different hues. This monochromatic approach preserves the serious, utility-focused tone of the application and prevents the "rainbow effect" that occurs when metadata competes with syntax-highlighted code.

| Token Name | Usage Context | Hex Code | Opacity | Rationale |
| :---- | :---- | :---- | :---- | :---- |
| **text-primary** | **Main Content, Code** | **\#E5E9F0** | **95%** | An off-white, "snow" color. Pure white (\#FFFFFF) is avoided because it creates excessive contrast against the charcoal background, leading to visual vibration. This slightly softened white ensures comfortable reading for long periods.4 |
| **text-secondary** | **Metadata, Timestamps** | **\#D8DEE9** | **70%** | A muted gray-blue used for secondary information like "Copied 2m ago" or keyboard shortcuts. The blue undertone links it visually to the background, pushing it further back in the visual plane.4 |
| **text-tertiary** | **Placeholders, Icons** | **\#4C566A** | **45%** | Low-contrast text for empty states ("Waiting for clipboard...") or decorative icons that do not require immediate attention. |
| **text-accent** | **Links, Active Toggles** | **\#88C0D0** | **100%** | A cyan-teal characteristic of modern AI interfaces. This color provides a "digital" pop for interactive elements (like the "Smart Scrub" button) without the aggression of a standard royal blue. It signals "action" and "intelligence".6 |

#### **1.1.3 Syntax Highlighting: The "Midnight Candy" Aesthetic**

Since Floating MD functions as a bridge for AI-generated content—much of which is Markdown code blocks—the syntax highlighting must be both beautiful and functional. Standard terminal themes (like distinct greens on black) often feel archaic. We will adopt a **"Midnight Candy"** theme: a collection of vibrant, soft pastels that pop against the deep charcoal background. This aligns with the specialized themes found in high-end editors like VS Code's "Claude" or "Linear" themes, creating a sense of continuity between the AI chat and the editor.7

* **Keywords (def, function, class):** **\#C79BFF** (Soft Purple). This color is distinctive but not jarring, often used for control flow keywords in modern themes.7  
* **Strings ("text"):** **\#86E89A** (Mint Green). Green is traditionally used for strings, but this specific mint shade offers superior contrast and readability against \#1C1D21 compared to a standard dark green.7  
* **Functions (print(), scrub()):** **\#7AB8FF** (Cornflower Blue). This links visually to the text-accent color, subtly reinforcing the idea that functions are "actions".7  
* **Comments (// TODO):** **\#6F6F78** (Steel Gray). Comments are pushed to the background visual layer, ensuring they do not distract from the executable logic.8  
* **Numbers/Constants:** **\#FFDF61** (Soft Gold). High visibility for data points, ensuring that hardcoded values stand out during quick scanning.7

### **1.2 Typography Stack: The "Global Modern" Pairing**

The typographic challenge for Floating MD is unique and significant: the system must pair a hyper-modern, geometric Latin font (to match the "Manus" look) with an Arabic font that maintains legibility in narrow layouts without breaking the geometric harmony of the interface. This requires a careful balancing of x-heights, stroke widths, and typographic character.

#### **1.2.1 The Primary Latin Typeface: Geist Sans**

While **Inter** has long been the industry standard for UI design 9, our research points to **Geist Sans** (by Vercel) as the superior choice for this specific "utility" context.

* **Rationale for Geist:** Geist is designed specifically for *developer tools* and interfaces with high information density. It is slightly more "technical" and "Swiss" than Inter, which has become ubiquitous to the point of genericism. Geist embodies the precision of the "action engine" philosophy behind Manus.11  
* **Metric Compatibility:** Geist features a tall x-height and open apertures, which are critical for readability at the small font sizes (12px-14px) typical of a clipboard history list. It renders crisply on Windows sub-pixel rendering engines.12  
* **Fallback Strategy:** Should technical constraints prevent the embedding of Geist, **Inter** remains the gold standard fallback, ensuring the interface retains its modern character.9

#### **1.2.2 The Arabic/RTL Typeface: IBM Plex Sans Arabic**

For Arabic text, **IBM Plex Sans Arabic** is the definitive recommendation, superior to popular alternatives like Cairo or Almarai for this specific application.14

* **The "Hybrid" Advantage:** Arabic typography in UI often struggles between tradition and modernity. Traditional "Naskh" fonts are legible but look outdated in a sleek UI. Geometric "Kufic" fonts (like Cairo) look modern but suffer from poor legibility in long blocks of text or code due to their rigid shapes. **IBM Plex Sans Arabic** is a "Grotesque" style Naskh hybrid. It balances traditional calligraphy rules—necessary for the fluid recognition of Arabic word shapes—with geometric, rationalized strokes that match the Latin counterpart.14  
* **Design Harmony:** It was explicitly designed by IBM to harmonize with code and technical UIs. Its stroke modulation and terminals mirror the logic of the Latin Plex (and by extension, Geist), ensuring that a mixed-language interface feels like a single cohesive voice rather than two disjointed fonts.14  
* **Technical Fit:** It supports a wide range of weights and has excellent "teeth" definition (the small vertical strokes in letters like *seen* and *sheen*). This clarity is critical for the "Smart Scrub" feature, where users need to distinguish character details to verify that code has been cleaned correctly.14

#### **1.2.3 Solving Mixed-Direction Inline Code**

One of the most persistent visual problems in RTL interfaces is the display of LTR (Left-to-Right) code snippets within RTL (Right-to-Left) Arabic text. Inserting code like print("hello") inside an Arabic sentence often causes the punctuation to "jump" to the wrong side or the text direction to invert unexpectedly.17  
To solve this, Floating MD will implement a **Bidirectional Isolation \+ "Pill" Styling** strategy.

* **CSS Logic:** We must strictly wrap all inline code in \<bdi\> (Bidirectional Isolation) tags or use unicode-bidi: isolate; in the CSS. This forces the browser to treat the code snippet as a self-contained LTR island, immune to the surrounding RTL flow.18  
* **Visual Styling:** Inline code must look like a distinct object to forgive any alignment awkwardness. We will apply a distinct background and border to these elements.  
  CSS  
  code {  
      font-family: 'Geist Mono', 'IBM Plex Mono', monospace;  
      background-color: rgba(255, 255, 255, 0.08); /\* Subtle highlight \*/  
      border: 1px solid rgba(255, 255, 255, 0.1); /\* Define edges \*/  
      border-radius: 4px; /\* Soften the block \*/  
      padding-inline: 4px; /\* Logical padding that flips for RTL \[19\] \*/  
      direction: ltr; /\* Force code to always read LTR \*/  
      unicode-bidi: isolate; /\* Prevent Arabic context from scrambling code \*/  
      vertical-align: middle; /\* Align with Arabic baseline \*/  
  }

* **Typographic Scaling:** Arabic scripts generally appear visually smaller than Latin scripts at the same point size due to their large loop structures and ascenders/descenders. To ensure visual parity, we must apply a **110% scale factor** to the Arabic font relative to Geist. For example, if Geist is set to 14px, IBM Plex Sans Arabic should be set to approx. 15.4px (or 1.1em).20

## **2\. Component Layouts: Solving the "Narrow Dock" Architecture**

Phase 1 established a strict physical constraint: a stationary dock, approximately 80px wide in its collapsed state, expanding to 300px. This form factor presents a significant challenge for RTL languages.

### **2.1 The RTL Verticality Problem**

The standard design pattern for clipboard managers is a vertical list of text snippets. However, this pattern fails catastrophically in Arabic. Arabic script is cursive and horizontally connected; words cannot be hyphenated or broken arbitrarily without rendering them illegible. In a narrow 80px or even 150px column, a single long Arabic word would either overflow, break strictly (ruining the letter connections), or require horizontal scrolling—all of which degrade the user experience.21

### **2.2 Evaluation of Layout Options**

To address this, we evaluated three potential layout patterns against the constraints of the Manus aesthetic and RTL legibility requirements.

* **Option A: Floating Cards (The "Drawer" Model)**  
  * *Concept:* The dock remains a thin strip of icons. Hovering over an icon triggers a wider "drawer" or card (300px+) to slide out from the side, containing the text.  
  * *Analysis:* While this maximizes reading width, "sliding" animations can be spatially confusing in a bidirectional OS environment. If the dock is on the *left* edge but the OS is in RTL mode, the slide direction might feel counter-intuitive. Furthermore, it requires significant mouse travel to interact with the content inside the drawer.22  
* **Option B: Horizontal Pills/Chips (The "Tag" Model)**  
  * *Concept:* Items are stacked vertically but represented as "Pills" that scroll *horizontally* inside the vertical container.  
  * *Analysis:* This solves the line-breaking issue by allowing text to flow on a single line. However, it introduces "scroll-ception" (horizontal scrolling inside a vertical list), which is widely regarded as an ergonomic failure. Users find dual-axis scrolling frustrating and imprecise.23  
* **Option C: Minimalist Icons \+ "Chat Bubble" Expansion (The "Manus" Hybrid)**  
  * *Concept:* The dock displays *only* metadata icons or extremely truncated pills (e.g., "Code...", "Text..."). Hovering an item instantly expands a "Chat Bubble" style card adjacent to the dock, displaying the full content.  
  * *Analysis:* This approach mimics the Manus chat interface, where content is contained in bubbles. It keeps the dock stationary and clean while allowing full-width reading for Arabic without permanently occupying screen space.

### **2.3 Recommendation: The "Contextual Hover-Bubble"**

We recommend a refined version of **Option C**, creating a layout explicitly optimized for the scanning behavior of power users.  
**The "Bubble" Layout Specification:**

1. **The Anchor (Dock Item):**  
   * The dock list does *not* attempt to display the text content. Instead, it serves as a timeline of events.  
   * Each item displays a **Type Icon** (e.g., \< \> for code, ¶ for text, IMG for images) and a **Time Label** ("2m", "Now").  
   * *Rationale:* Users typically recall clipboard history by *time* and *type* ("That code snippet I copied 2 minutes ago"). By removing the text from the dock itself, we bypass the narrow column legibility issue entirely.  
2. **The Reveal (Hover Card):**  
   * **Trigger:** Hovering over an anchor for more than 200ms (a "deliberate hover") triggers the expansion. The delay prevents accidental flickering as the mouse passes over the dock.25  
   * **Appearance:** A card appears *adjacent* to the dock (e.g., to the left if the dock is on the right edge).  
   * **Card Styling:**  
     * **Background:** bg-surface (\#2E3440).  
     * **Border:** border-subtle (\#3E4451) with a soft cyan glow.  
     * **Width:** Auto-expanding up to 400px. This width is sufficient to display comprehensive lines of Arabic text or code without awkward wrapping.  
     * **Typography:** IBM Plex Sans Arabic at 16px, optimized for maximum legibility.20  
   * **Interaction:** The user can move the mouse *into* the card to scroll its content or click actions (Copy, Edit, Scrub). This "bridge" interaction allows the card to persist as long as the user is interacting with it.  
3. **RTL Optimization & Mirroring:**  
   * If the dock is placed on the **Right** edge (standard for Arabic Windows layouts), the card pops out to the **Left**.  
   * If the dock is on the **Left** edge, the card pops out to the **Right**.  
   * This automatic "mirroring" behavior is non-negotiable for a native feel. It respects the user's mental model of where "space" is available on the screen.17

## **3\. Feature UX Flows: The "Smart Scrub" Experience**

The "Smart Scrub" feature addresses a core friction point in the AI-assisted workflow: "preamble pollution." When users copy code or text from an LLM, it often includes conversational filler (e.g., "Certainly\! Here is the code you requested..."). The goal of this feature is to strip this noise effectively, making the process feel like "magic" while ensuring user trust.

### **3.1 Interaction Psychology: Magic vs. Control**

Users generally trust AI to *generate* content, but they fear AI *deleting* or modifying content destructively. If Floating MD auto-cleans text silently, users may panic, fearing that critical code or context was lost. Therefore, the UX must be **Optimistic but Reversible**. It should assume the user wants clean text but provide an immediate, visible safety net.

### **3.2 The "Sparkle" Pattern**

Research indicates that the "Sparkle" icon (✨) has become the universally recognized signifier for "AI Enhancement" or "Magic".27 We will leverage this existing mental model.  
**The UX Flow:**

1. **Detection:** When text is copied to the clipboard, Floating MD analyzes it in the background using lightweight regex or heuristics. If it detects common AI patterns (Markdown blocks preceded by conversational text like "Here is...", "In conclusion"), it flags the item in the history.  
2. **The Indicator:** The corresponding item in the dock receives a subtle, pulsing **Gold/Purple Sparkle Icon** overlay. This signals to the user: "I have detected AI residue, and I can clean this for you."  
3. **The Trigger:**  
   * *User Action:* The user clicks the Sparkle button (or uses a dedicated hotkey).  
   * *Animation:* A "shimmer" effect sweeps across the text card (implemented via a CSS gradient mask animation). This visualizes the "cleaning" process.29  
   * *Result:* The preamble and postscript text vanish, leaving only the core content (the Code block or the direct Answer).  
4. **The Safety Net (Undo Toast):**  
   * Immediately after the scrubbing action, a **Toast Notification** appears at the bottom of the dock.  
   * *Text:* "AI Preamble Removed."  
   * *Action:* **"Undo"** (Text button).  
   * *Duration:* 5 seconds.30  
   * *Rationale:* This gives the user confidence. They know that if the AI was too aggressive and removed something important, they can restore the original text with a single click.31

### **3.3 Visualizing the "Clean" State**

Once an item has been scrubbed, its visual state changes to "Verified/Clean" to prevent repetitive actions.

* **Border:** The card border changes from border-subtle to border-accent (Cyan/Teal).  
* **Icon:** The Sparkle icon transforms into a solid **Checkmark** (✓).  
* This persistent feedback loop confirms to the user that "this snippet is ready to paste," reducing the cognitive load of checking the text manually.32

## **4\. Focus State Feedback: The "Alive" Interface**

A floating utility exists in a unique and challenging state within the Windows OS environment: it is visible but not always "active" (focused). Technically, to prevent stealing focus from the user's primary application (like Word or VS Code), the window often uses the WS\_EX\_NOACTIVATE style.33 However, this can make the app feel unresponsive or "dead." The UI must clearly and visually communicate its state to the user: *"Am I just watching, or am I ready for you to type?"*

### **4.1 The "Passive" State (Viewer Mode)**

* **Context:** The dock is floating on the edge of the screen, and the user is actively typing in another application (e.g., VS Code).  
* **Visuals:**  
  * **Opacity:** **85%**. This allows the desktop wallpaper or windows behind the dock to bleed through slightly, reinforcing its nature as a lightweight, background utility.  
  * **Borders:** border-subtle (\#3E4451).  
  * **Shadows:** None (flat appearance).  
  * **Cursor:** Standard arrow.  
* **Meaning:** "I am a background utility. I am monitoring the clipboard, but I will not intercept your keystrokes."

### **4.2 The "Active" State (Editor Mode)**

* **Context:** The user has clicked on the dock or pressed the global hotkey to activate the Markdown editor or search the clipboard history.  
* **Visuals:**  
  * **Opacity:** **100%**. The interface becomes a solid, opaque surface, signaling that it is now the primary workspace.  
  * **Border Glow:** A **2px Cyan Glow** (box-shadow: 0 0 10px \#88C0D0) animates in. Since the window lacks a native OS title bar (which typically changes color to indicate focus), this "neon" glow serves as the proxy for window activation.34  
  * **Elevation:** A deep drop shadow (box-shadow: 0 20px 25px \-5px rgba(0, 0, 0, 0.5)) lifts the dock visually "above" the desktop plane.35  
  * **Accent Line:** A 3px colored bar appears at the top (or side) of the active panel, reinforcing the "On" state.  
* **Meaning:** "I am now the active window. Any keys you type will be captured here."

### **4.3 Transition Animation**

* **Timing:** 200ms ease-out.  
* **Property:** transform: scale(1.02).  
* **Effect:** Upon activation, the dock literally "swells" slightly towards the user, mimicking the physical behavior of a button being pressed or a card being picked up.36 This tactile feedback is crucial for "Fitts's Law" targeting—it confirms to the user's peripheral vision that they have successfully acquired the target without needing to look directly at the cursor.

## **5\. Implementation Technicalities (Design Handoff)**

This section translates the visual decisions into actionable CSS and technical specifications for the engineering team.

### **5.1 CSS Variables (Theme Definition)**

CSS

:root {  
  /\* Manus Dark Theme Core \*/  
  \--bg-primary: \#1C1D21;   /\* Main Shell \- Deep Charcoal \*/  
  \--bg-surface: \#2E3440;   /\* Cards/Panels \- Slate Gray \*/  
  \--bg-hover:   \#3B4252;   /\* Interactive Hover State \*/  
    
  /\* Borders & Dividers \*/  
  \--border-subtle: \#3E4451;  
  \--border-focus:  \#88C0D0; /\* Cyan Glow for Active State \*/

  /\* Typography \- Geist Sans \+ IBM Plex Sans Arabic \*/  
  \--font-latin:  'Geist Sans', 'Inter', system-ui, sans-serif;  
  \--font-arabic: 'IBM Plex Sans Arabic', 'Segoe UI', sans-serif;  
  \--font-mono:   'Geist Mono', 'IBM Plex Mono', monospace;

  /\* Text Colors \*/  
  \--text-primary:   \#E5E9F0; /\* Off-white for main text \*/  
  \--text-secondary: \#D8DEE9; /\* Muted blue-gray for metadata \*/  
  \--text-muted:     \#4C566A; /\* Low contrast for placeholders \*/

  /\* Syntax Highlighting (Midnight Candy) \*/  
  \--syntax-kwd:    \#C79BFF; /\* Purple \- Keywords \*/  
  \--syntax-str:    \#86E89A; /\* Green \- Strings \*/  
  \--syntax-func:   \#7AB8FF; /\* Blue \- Functions \*/  
  \--syntax-num:    \#FFDF61; /\* Gold \- Numbers \*/  
}

### **5.2 RTL/Bidirectional CSS Strategy**

To ensure the layout works seamlessly for Arabic users without the need to maintain two separate codebases, the engineering team must strictly utilize **Logical Properties** in CSS.37

* **Prohibited Properties:** margin-left, margin-right, padding-left, padding-right, border-left, border-right, left, right.  
* **Required Properties:**  
  * margin-inline-start / margin-inline-end  
  * padding-inline-start / padding-inline-end  
  * border-inline-start / border-inline-end  
  * inset-inline-start / inset-inline-end  
* **Why:** When the Windows OS sets the application context to RTL, these properties automatically flip the layout geometry. The "Contextual Hover-Bubble" will automatically appear on the correct side of the dock (Left for Right-docked, Right for Left-docked) without requiring any additional JavaScript logic or manual overrides.

### **5.3 The "Glass" Effect (Optional Enhancement)**

If the target Windows environment supports **Mica** or **Acrylic** effects (common in Windows 11), the \--bg-primary color should be applied with **80% opacity** combined with a **backdrop-blur(20px)** filter. This aligns the app with the "Liquid Glass" trend in modern OS design 38, helping the utility feel like a native extension of the desktop environment rather than a foreign object.

## **6\. Conclusion**

The visual system defined in this report does not shout; it whispers. By adopting the **Manus AI** deep charcoal palette and the **Geist/IBM Plex** typography stack, Floating MD achieves a professional, expert-level aesthetic that inspires confidence. The **Contextual Hover-Bubble** layout elegantly solves the RTL legibility crisis inherent in narrow docks, while the **Smart Scrub** "sparkle" interaction transforms a mundane text-cleaning task into a moment of delight. This specification provides a complete, robust roadmap for Phase 2, ensuring that Floating MD is not just a utility, but a refined instrument for the AI-augmented workflow.

#### **المصادر التي تم الاقتباس منها**

1. Dark theme \- Material Design, تم الوصول بتاريخ ‎يناير 22, 2026، [https://m2.material.io/design/color/dark-theme.html](https://m2.material.io/design/color/dark-theme.html)  
2. Dark mode UI design – 7 best practices \- Atmos Style, تم الوصول بتاريخ ‎يناير 22, 2026، [https://atmos.style/blog/dark-mode-ui-best-practices](https://atmos.style/blog/dark-mode-ui-best-practices)  
3. Dark UI Color Palette, تم الوصول بتاريخ ‎يناير 22, 2026، [https://www.color-hex.com/color-palette/46857](https://www.color-hex.com/color-palette/46857)  
4. UI \- Dark Mode 1 Color Palette, تم الوصول بتاريخ ‎يناير 22, 2026، [https://www.color-hex.com/color-palette/1056617](https://www.color-hex.com/color-palette/1056617)  
5. Does anyone know the new hex code for dark mode's background? : r/discordapp \- Reddit, تم الوصول بتاريخ ‎يناير 22, 2026، [https://www.reddit.com/r/discordapp/comments/11ywtz4/does\_anyone\_know\_the\_new\_hex\_code\_for\_dark\_modes/](https://www.reddit.com/r/discordapp/comments/11ywtz4/does_anyone_know_the_new_hex_code_for_dark_modes/)  
6. Dark Color Palettes \- Coolors, تم الوصول بتاريخ ‎يناير 22, 2026، [https://coolors.co/palettes/trending/dark](https://coolors.co/palettes/trending/dark)  
7. Claude VSCode Theme \- Open VSX Registry, تم الوصول بتاريخ ‎يناير 22, 2026، [https://open-vsx.org/extension/AlvinUnreal/claude-vscode-theme](https://open-vsx.org/extension/AlvinUnreal/claude-vscode-theme)  
8. Claude Theme \- Visual Studio Marketplace, تم الوصول بتاريخ ‎يناير 22, 2026، [https://marketplace.visualstudio.com/items?itemName=Lumidew.claude-color-theme](https://marketplace.visualstudio.com/items?itemName=Lumidew.claude-color-theme)  
9. Designers, if you could choose only one font to use for all your designs, which would it be and why? : r/UXDesign \- Reddit, تم الوصول بتاريخ ‎يناير 22, 2026، [https://www.reddit.com/r/UXDesign/comments/1hkdc90/designers\_if\_you\_could\_choose\_only\_one\_font\_to/](https://www.reddit.com/r/UXDesign/comments/1hkdc90/designers_if_you_could_choose_only_one_font_to/)  
10. Arabic Language Support · Issue \#391 · rsms/inter \- GitHub, تم الوصول بتاريخ ‎يناير 22, 2026، [https://github.com/rsms/inter/issues/391](https://github.com/rsms/inter/issues/391)  
11. We're moving from Inter to Geist. \- Peerlist, تم الوصول بتاريخ ‎يناير 22, 2026، [https://peerlist.io/scroll/post/ACTH6AJMBRJAQQRKD2OQGL69G7DG98](https://peerlist.io/scroll/post/ACTH6AJMBRJAQQRKD2OQGL69G7DG98)  
12. Geist \- Google Fonts, تم الوصول بتاريخ ‎يناير 22, 2026، [https://fonts.google.com/specimen/Geist](https://fonts.google.com/specimen/Geist)  
13. Inter Font Pairings (Google fonts) & Alternatives \- MaxiBestOf, تم الوصول بتاريخ ‎يناير 22, 2026، [https://maxibestof.one/typefaces/inter](https://maxibestof.one/typefaces/inter)  
14. IBM Plex Sans Arabic \- Google Fonts, تم الوصول بتاريخ ‎يناير 22, 2026، [https://fonts.google.com/specimen/IBM+Plex+Sans+Arabic](https://fonts.google.com/specimen/IBM+Plex+Sans+Arabic)  
15. 10 Arabic Fonts Every UX Designer Should Know in 2025 \- Ahmed Elramlawy, تم الوصول بتاريخ ‎يناير 22, 2026، [https://ahmedelramlawy.com/10-arabic-fonts-every-ux-designer-should-know-in-2025/](https://ahmedelramlawy.com/10-arabic-fonts-every-ux-designer-should-know-in-2025/)  
16. The package of IBM's typeface, IBM Plex. \- GitHub, تم الوصول بتاريخ ‎يناير 22, 2026، [https://github.com/IBM/plex](https://github.com/IBM/plex)  
17. Bidirectionality \- Material Design, تم الوصول بتاريخ ‎يناير 22, 2026، [https://m2.material.io/design/usability/bidirectionality.html](https://m2.material.io/design/usability/bidirectionality.html)  
18. Inline markup and bidirectional text in HTML \- W3C, تم الوصول بتاريخ ‎يناير 22, 2026، [https://www.w3.org/International/articles/inline-bidi-markup/index.en.html](https://www.w3.org/International/articles/inline-bidi-markup/index.en.html)  
19. 50 Beautiful Website Color Schemes & CSS Hex Codes (2026) \- Hook Agency, تم الوصول بتاريخ ‎يناير 22, 2026، [https://hookagency.com/blog/website-color-schemes-2020/](https://hookagency.com/blog/website-color-schemes-2020/)  
20. Stretchable kashida and Arabic text justification in LaTeX \- Uppercase Alif, تم الوصول بتاريخ ‎يناير 22, 2026، [http://andreasmhallberg.github.io/stretchable-kashida/](http://andreasmhallberg.github.io/stretchable-kashida/)  
21. Hover Cards \- Ui-Layouts, تم الوصول بتاريخ ‎يناير 22, 2026، [https://www.ui-layouts.com/components/hover-cards](https://www.ui-layouts.com/components/hover-cards)  
22. The Dilemma of Horizontal and Vertical Scroll in UX Design\! | by Mehekk Bassi | Prototypr, تم الوصول بتاريخ ‎يناير 22, 2026، [https://blog.prototypr.io/the-dilemma-of-horizontal-and-vertical-scroll-in-ux-design-6b3158bad461](https://blog.prototypr.io/the-dilemma-of-horizontal-and-vertical-scroll-in-ux-design-6b3158bad461)  
23. What are the pros and cons of mixing horizontal scrolling with vertical scrolling?, تم الوصول بتاريخ ‎يناير 22, 2026، [https://ux.stackexchange.com/questions/29180/what-are-the-pros-and-cons-of-mixing-horizontal-scrolling-with-vertical-scrollin](https://ux.stackexchange.com/questions/29180/what-are-the-pros-and-cons-of-mixing-horizontal-scrolling-with-vertical-scrollin)  
24. Vertical Dropdown Menu design pattern, تم الوصول بتاريخ ‎يناير 22, 2026، [https://ui-patterns.com/patterns/VerticalDropdownMenu](https://ui-patterns.com/patterns/VerticalDropdownMenu)  
25. 7 Examples of Right to Left Design from JetRuby, تم الوصول بتاريخ ‎يناير 22, 2026، [https://jetruby.com/blog/creating-an-impressive-right-to-left-design/](https://jetruby.com/blog/creating-an-impressive-right-to-left-design/)  
26. Rise of the AI Sparkle Icon \- Google Design, تم الوصول بتاريخ ‎يناير 22, 2026، [https://design.google/library/ai-sparkle-icon-research-pozos-schmidt](https://design.google/library/ai-sparkle-icon-research-pozos-schmidt)  
27. AI Buttons \- Innovaccer, تم الوصول بتاريخ ‎يناير 22, 2026، [https://design.innovaccer.com/components/AIButton/usage/](https://design.innovaccer.com/components/AIButton/usage/)  
28. The 55 Best CSS Button Hover Effects You Can Use Too \- Slider Revolution, تم الوصول بتاريخ ‎يناير 22, 2026، [https://www.sliderrevolution.com/resources/css-button-hover-effects/](https://www.sliderrevolution.com/resources/css-button-hover-effects/)  
29. How long should a toast message with 'undo' appear? \- User Experience Stack Exchange, تم الوصول بتاريخ ‎يناير 22, 2026، [https://ux.stackexchange.com/questions/116634/how-long-should-a-toast-message-with-undo-appear](https://ux.stackexchange.com/questions/116634/how-long-should-a-toast-message-with-undo-appear)  
30. Toast Messages vs. Snackbars, Banners, and Push Notifications \- Courier, تم الوصول بتاريخ ‎يناير 22, 2026، [https://www.courier.com/blog/what-is-a-toast-message](https://www.courier.com/blog/what-is-a-toast-message)  
31. Clipboard Text Cleaner \- University of Mississippi Foundation, تم الوصول بتاريخ ‎يناير 22, 2026، [https://www.umfoundation.com/clipboard/](https://www.umfoundation.com/clipboard/)  
32. 01-clude research.txt  
33. CSS/HTML: Create a glowing border around an Input Field \- Stack Overflow, تم الوصول بتاريخ ‎يناير 22, 2026، [https://stackoverflow.com/questions/5670879/css-html-create-a-glowing-border-around-an-input-field](https://stackoverflow.com/questions/5670879/css-html-create-a-glowing-border-around-an-input-field)  
34. Focusing on focus states. Some things I've learned about… | by Dave House | Medium, تم الوصول بتاريخ ‎يناير 22, 2026، [https://iknowdavehouse.medium.com/focusing-on-focus-states-7c64089d805c](https://iknowdavehouse.medium.com/focusing-on-focus-states-7c64089d805c)  
35. Components state a friendly guideline on how to use it (UX Blueprint 07\) \- Medium, تم الوصول بتاريخ ‎يناير 22, 2026، [https://medium.com/design-bootcamp/ux-blueprint-07-components-state-a-friendly-guideline-on-how-to-use-it-5ad549de05f1](https://medium.com/design-bootcamp/ux-blueprint-07-components-state-a-friendly-guideline-on-how-to-use-it-5ad549de05f1)  
36. Stop Fighting RTL Layouts: Use CSS Logical Properties for Better Design \- Medium, تم الوصول بتاريخ ‎يناير 22, 2026، [https://medium.com/nerd-for-tech/stop-fighting-rtl-layouts-use-css-logical-properties-for-better-design-236edec711fa](https://medium.com/nerd-for-tech/stop-fighting-rtl-layouts-use-css-logical-properties-for-better-design-236edec711fa)  
37. Notable User Interface Changes to Expect in macOS 26 \- MacTLC, تم الوصول بتاريخ ‎يناير 22, 2026، [https://mactlc.com/post/notable-user-interface-ch](https://mactlc.com/post/notable-user-interface-ch)