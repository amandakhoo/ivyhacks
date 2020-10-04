import React from 'react'
import TodoItem from './components/TodoItem'
import AddTodo from './components/AddTodo'
import * as data from './demo.json'

const App: React.FC = () => {

  return (
    <main className='App'>
      <h1>Search Results</h1>
      <AddTodo query={data.terms} />
      {data.results.map((todo: string) => (
        <TodoItem
          key={todo}
          result={todo}
        />
      ))}
    </main>
  )
}

export default App
