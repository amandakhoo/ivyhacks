import React from 'react'

type Props = {
  result: string
}

const Todo: React.FC<Props> = ({ result }) => {
  return (
    <div className='Card'>
      <div className='Card--text'>
        <span className='checkTodo'>{result}</span>
      </div>
    </div>
  )
}

export default Todo
