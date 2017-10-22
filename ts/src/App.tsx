import * as React from 'react'
import { Link, Route, Switch, match } from 'react-router-dom'
import Editions from './ethics/Editions'
import Home from './Home'
import Ethics from './ethics/Ethics'
import CreateEdition from './ethics/CreateEdition'
import styles = require('./shell.scss')

interface StateProps {}

interface DispatchProps {}

interface OwnProps {
    match: match<{}>
}

type Props = StateProps & DispatchProps & OwnProps

export const App = (props: Props) => (
  <div className={styles['app-container']}>
    <div className={styles.banner}>
      <Link className={styles.title} to='/'>ET ÇA NE FAIT QUE COMMENCER</Link>
    </div>
    <div className={styles.content}>
      <div>
        <Switch>
          <Route exact path='/' component={Home} />
          <Route exact path='/ethics' component={Editions} />
          <Route path='/ethics/create' component={CreateEdition} />
          <Route path='/ethics/:editionSlug' component={Ethics} />
        </Switch>
      </div>
    </div>
  </div>)

export default App
