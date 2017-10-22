import * as React from 'react'
import { bindActionCreators } from 'redux'
import { Link, match } from 'react-router-dom'
import * as api from 'api-types'
import styles = require('./home.scss')

interface StateProps {}

interface DispatchProps {}

interface OwnProps {
  match: match<{}>
}

type Props = StateProps & DispatchProps & OwnProps

export const Home = (props: Props) =>
    <div className={styles.home}>
        <p>
            Choose your adventure:
        </p>
        <div className={styles.books}>
          <Link className='book' to='/ethica'>
              Ethica more geometrico demonstrata
          </Link>
        </div>
        <p>
        </p>
    </div>

export default Home
