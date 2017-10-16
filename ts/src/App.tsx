import * as React from 'react'
import { AppState } from './reducers'
import * as api from 'rpc/repository_pb'
import { Link, Route, Switch, match } from 'react-router-dom'
import Editions from './Editions'
import Home from './Home'
import Ethics from './Ethics'

interface StateProps {}

interface DispatchProps {}

interface OwnProps {
    match: match<{}>
}

type Props = StateProps & DispatchProps & OwnProps

export const App = (props: Props) => (
    <div>
        <div>
            <Link to='/'>Home</Link>
        </div>
        <div className='container'>
            <Switch>
                <Route exact path='/' component={Home} />
                <Route exact path='/ethica' component={Editions} />
                <Route path='/ethica/:editionSlug' component={Ethics} />
            </Switch>
        </div>
    </div>)

export default App
