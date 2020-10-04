import React from 'react'

type Props = {
  query: string
}

const AddTodo: React.FC<Props> = ({ query }) => {

  return (
    <form className='Form' >
      <div>
        <div>
          <label htmlFor='name'>Name</label>
          <input type='text' id='name' defaultValue={query} />
        </div>
      </div>
      <button disabled={query !== ""} >Search</button>
    </form>
  )
}

export default AddTodo
