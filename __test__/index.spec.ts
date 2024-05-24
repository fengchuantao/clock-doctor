import test from 'ava'

import { checkAndSet } from '../index'

test('sync function from native code', (t) => {
  // t.is(checkAndSet(fixture), fixture + 100)
  t.truthy(checkAndSet())
})
