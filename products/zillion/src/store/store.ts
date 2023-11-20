import { configureStore, getDefaultMiddleware, Middleware } from '@reduxjs/toolkit'

import userReducer from '../store/userSlice'
import blockchainReducer from '../store/blockchainSlice'
import stakingReducer from '../store/stakingSlice'
import sagaMiddleware, { startSagas } from '../saga'

const IS_DEV_ENV = !process.env.NODE_ENV || process.env.NODE_ENV === "development"
const middlewares: Middleware[] = [...getDefaultMiddleware(), sagaMiddleware]

const store = configureStore({
    reducer: {
        user: userReducer,
        blockchain: blockchainReducer,
        staking: stakingReducer,
    },
    devTools: IS_DEV_ENV,
    middleware: middlewares
})

startSagas()

export default store

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch