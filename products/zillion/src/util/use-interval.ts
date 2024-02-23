import { useEffect, useRef } from 'react';

export function useInterval(callback: any, callerRef: any, delay: any) {
    const savedCallback: any = useRef();
    // remember the latest callback
    useEffect(() => {
        savedCallback.current = callback;
    }, [callback]);

    // set up the interval
    useEffect(() => {
        function tick() {
            savedCallback.current();
        }
        if (delay !== null) {
            // caller ref to prevent updating state internally
            // when component is unmounted
            callerRef.current = true;
            const id = setInterval(tick, delay);
            return () => {
                callerRef.current = false;
                clearInterval(id);
            };
        }
    }, [callback, callerRef, delay]);
}