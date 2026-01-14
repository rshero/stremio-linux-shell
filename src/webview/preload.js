const createIpc = () => {
    let listeners = [];

    globalThis.IPC_SENDER = (data) =>  {
        listeners.forEach((listener) => {
            listener({ data });
        });
    };

    const postMessage = (data) => {
        globalThis.IPC_RECEIVER(data);
    };

    const addEventListener = (name, listener) => {
        if (name !== 'message')
            throw Error('Unsupported event');

        listeners.push(listener);
    };

    const removeEventListener = (name, listener) => {
        if (name !== 'message')
            throw Error('Unsupported event');

        listeners = listeners.filter((it) => it !== listener);
    };

    return {
        postMessage,
        addEventListener,
        removeEventListener,
    };
};

window.ipc = createIpc();

// Backward compatibility
window.qt = {
    webChannelTransport: {
        send: window.ipc.postMessage,
    },
};

globalThis.chrome = {
    webview: {
        postMessage: window.ipc.postMessage,
        addEventListener: (name, listener) => {
            window.ipc.addEventListener(name, listener);
        },
        removeEventListener: (name, listener) => {
            window.ipc.removeEventListener(name, listener);
        },
    },
};

window.ipc.addEventListener('message', (message) => {
    window.qt.webChannelTransport.onmessage(message);
});

// Theme API for stremio-web
const createThemeApi = () => {
    let messageId = 1000;
    const pendingCallbacks = new Map();

    // Listen for theme responses
    window.ipc.addEventListener('message', (event) => {
        try {
            const message = JSON.parse(event.data);
            if (message && message.args && Array.isArray(message.args)) {
                const [eventName, data] = message.args;
                
                // Handle theme events
                if (eventName === 'theme-settings') {
                    const callbacks = pendingCallbacks.get('theme-settings');
                    if (callbacks) {
                        callbacks.forEach(cb => cb(data));
                        pendingCallbacks.delete('theme-settings');
                    }
                } else if (eventName === 'theme-file-content') {
                    const callbacks = pendingCallbacks.get('theme-file-content');
                    if (callbacks) {
                        callbacks.forEach(cb => cb(data));
                        pendingCallbacks.delete('theme-file-content');
                    }
                } else if (eventName === 'theme-file-list') {
                    const callbacks = pendingCallbacks.get('theme-file-list');
                    if (callbacks) {
                        callbacks.forEach(cb => cb(data));
                        pendingCallbacks.delete('theme-file-list');
                    }
                } else if (eventName === 'theme-write-result') {
                    const callbacks = pendingCallbacks.get('theme-write-result');
                    if (callbacks) {
                        callbacks.forEach(cb => cb(data));
                        pendingCallbacks.delete('theme-write-result');
                    }
                }
            }
        } catch (e) {
            // Not a JSON message or not a theme event
        }
    });

    const sendMessage = (name, data) => {
        const id = messageId++;
        const message = {
            id,
            type: 6,
            args: data !== undefined ? [name, data] : [name]
        };
        window.ipc.postMessage(JSON.stringify(message));
        return id;
    };

    const waitForResponse = (eventName) => {
        return new Promise((resolve) => {
            if (!pendingCallbacks.has(eventName)) {
                pendingCallbacks.set(eventName, []);
            }
            pendingCallbacks.get(eventName).push(resolve);
        });
    };

    return {
        // Check if running in shell
        isShell: true,

        // Get current theme settings (url and css)
        getSettings: async () => {
            sendMessage('theme-get-settings');
            return waitForResponse('theme-settings');
        },

        // Set theme URL
        setUrl: (url) => {
            sendMessage('theme-set-url', url);
        },

        // Set custom CSS
        setCss: (css) => {
            sendMessage('theme-set-css', css);
        },

        // Read a theme file from themes directory
        readFile: async (filename) => {
            sendMessage('theme-read-file', filename);
            return waitForResponse('theme-file-content');
        },

        // Write a theme file to themes directory
        writeFile: async (filename, content) => {
            sendMessage('theme-write-file', [filename, content]);
            return waitForResponse('theme-write-result');
        },

        // List available theme files
        listFiles: async () => {
            sendMessage('theme-list-files');
            return waitForResponse('theme-file-list');
        }
    };
};

window.stremioTheme = createThemeApi();

console.log('preload');
