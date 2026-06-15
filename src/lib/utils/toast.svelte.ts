export interface ToastItem {
  id: string;
  message: string;
  type: 'info' | 'success' | 'warning' | 'error';
  duration?: number;
}

class ToastManager {
  toasts = $state<ToastItem[]>([]);

  add(message: string, type: ToastItem['type'] = 'info', duration = 4000) {
    const id = Math.random().toString(36).substring(2, 9);
    this.toasts.push({ id, message, type, duration });

    if (duration > 0) {
      setTimeout(() => {
        this.remove(id);
      }, duration);
    }
  }

  remove(id: string) {
    this.toasts = this.toasts.filter(t => t.id !== id);
  }

  info(message: string, duration?: number) { this.add(message, 'info', duration); }
  success(message: string, duration?: number) { this.add(message, 'success', duration); }
  warning(message: string, duration?: number) { this.add(message, 'warning', duration); }
  error(message: string, duration?: number) { this.add(message, 'error', duration); }
}

export const toast = new ToastManager();
