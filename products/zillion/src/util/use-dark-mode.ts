import { useState, useEffect, useCallback } from 'react';
import { isStorageAvailable } from './use-local-storage';


const setTheme = (darkMode: boolean) => {
    if (darkMode) {
        document.documentElement.setAttribute('data-theme', 'dark');
    } else {
        document.documentElement.setAttribute('data-theme', 'light');
    }
}

const useDarkMode = (initialValue = true) => {
    const [darkMode, setDarkMode] = useState(() => {
        // try to load from local storage or if the document element has already been set
        try {
            // try to get from local storage
            const item = window.localStorage.getItem("dark-theme");

            // try to get from existing attribute
            const currTheme = document.documentElement.getAttribute('data-theme');

            if (item !== null) {
                const localStorageTheme = JSON.parse(item);

                // source of truth from local storage
                // fix flashing bug if default is dark mode but storage is light mode
                setTheme(localStorageTheme)
                return localStorageTheme;

            } else if (currTheme !== null && currTheme !== '') {
                // source of truth from document attribute
                var isDarkMode = true;
                if (currTheme === 'dark') {
                    isDarkMode = true;
                } else {
                    isDarkMode = false;
                }
                return isDarkMode;
            } else {
                return initialValue
            }

        } catch (err) {
            return initialValue;
        }
    });

    useEffect(() => {
        if (isStorageAvailable('localStorage')) {
            window.localStorage.setItem("dark-theme", JSON.stringify(darkMode));
        }
        
        if (darkMode) {
            document.documentElement.setAttribute('data-theme', 'dark');
        } else {
            document.documentElement.setAttribute('data-theme', 'light');
        }
    }, [darkMode]);

    return {
        value: darkMode,
        enable: useCallback(() => {
                    setDarkMode(true);
                    // document.documentElement.setAttribute('data-theme', 'dark');
                }, [setDarkMode]),
        disable: useCallback(() => {
                    setDarkMode(false);
                    // document.documentElement.setAttribute('data-theme', 'light');
                }, [setDarkMode]),
    }
}

export default useDarkMode;