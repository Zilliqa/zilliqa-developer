import createSagaMiddleware from 'redux-saga';
import stakingSaga from './stakingSaga';
import userSaga from './userSaga';

const sagaMiddleware = createSagaMiddleware();

export function startSagas() {
    sagaMiddleware.run(stakingSaga);
    sagaMiddleware.run(userSaga);
    // sagaMiddleware.run(preloadSaga);
    // sagaMiddleware.run(otherSaga)
}

export default sagaMiddleware;