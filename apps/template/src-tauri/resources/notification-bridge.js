(function() {
  const OriginalNotification = window.Notification;

  class TauriNotification {
    constructor(title, options = {}) {
      this.title = title;
      this.body = options.body || '';
      this.icon = options.icon || '';
      this.tag = options.tag || '';
      this.onclick = null;
      this.onclose = null;
      this.onerror = null;
      this.onshow = null;

      this._show();
    }

    async _show() {
      try {
        if (window.__TAURI__?.core?.invoke) {
          await window.__TAURI__.core.invoke('show_notification', {
            title: this.title,
            body: this.body
          });
          if (this.onshow) this.onshow();
        }
      } catch (error) {
        console.error('Failed to show notification:', error);
        if (this.onerror) this.onerror(error);
      }
    }

    close() {
      if (this.onclose) this.onclose();
    }

    static get permission() {
      return 'granted';
    }

    static async requestPermission() {
      return 'granted';
    }
  }

  window.Notification = TauriNotification;

  console.log('[WebBox] Notification bridge initialized');
})();
