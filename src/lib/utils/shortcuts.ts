type ShortcutCallback = (e: KeyboardEvent) => void;

interface Shortcut {
  key: string;            // e.g. 'n', 'b', ',', '1', '2'
  metaOrCtrl: boolean;    // Cmd (Mac) or Ctrl (Windows)
  shift?: boolean;
  callback: ShortcutCallback;
  description: string;
}

class ShortcutManager {
  private shortcuts: Shortcut[] = [];
  private isListening = false;

  register(
    key: string,
    callback: ShortcutCallback,
    options: { metaOrCtrl?: boolean; shift?: boolean; description?: string } = {}
  ) {
    this.shortcuts.push({
      key: key.toLowerCase(),
      metaOrCtrl: options.metaOrCtrl ?? true,
      shift: options.shift ?? false,
      callback,
      description: options.description ?? ''
    });
  }

  start() {
    if (this.isListening) return;
    window.addEventListener('keydown', this.handleKeyDown);
    this.isListening = true;
  }

  stop() {
    if (!this.isListening) return;
    window.removeEventListener('keydown', this.handleKeyDown);
    this.isListening = false;
  }

  private handleKeyDown = (e: KeyboardEvent) => {
    // Skip if typing in an input or textarea
    const activeEl = document.activeElement;
    const isTyping = activeEl && (
      activeEl.tagName === 'INPUT' || 
      activeEl.tagName === 'TEXTAREA' || 
      (activeEl as HTMLElement).isContentEditable
    );

    const matchKey = e.key.toLowerCase();
    const hasMeta = e.metaKey || e.ctrlKey;
    const hasShift = e.shiftKey;

    for (const shortcut of this.shortcuts) {
      // Don't trigger standard global shortcuts while typing in inputs
      // (Unless it's something like Command+Enter, but those are handled locally by components)
      if (isTyping && shortcut.key !== 'enter') {
        continue;
      }

      if (
        shortcut.key === matchKey &&
        shortcut.metaOrCtrl === hasMeta &&
        shortcut.shift === hasShift
      ) {
        e.preventDefault();
        shortcut.callback(e);
        break;
      }
    }
  };
}

export const shortcuts = new ShortcutManager();
export type { Shortcut };
