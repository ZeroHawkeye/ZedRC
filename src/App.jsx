import "./App.css";
import { message, Input, Button } from "antd";
import { FolderOpenOutlined } from "@ant-design/icons";
import { open } from '@tauri-apps/api/dialog';
import { useState } from "react";
import { invoke } from "@tauri-apps/api";

function App() {
  const [path, setPath] = useState('')
  const [messageApi, contextHolder] = message.useMessage();

  const openFile = async () => {
    const selected = await open({
      filters: [{
        name: 'zed',
        extensions: ['exe'],
      }],
      directory: false,
      multiple: false,
    });
    setPath(selected)
  }
  const regRight = () => {
    if (path.trim() === "") {
      messageApi.error('Please select a path')
      return
    }
    console.log(path)
    invoke('reg_right', { path }).then(() => {
      messageApi.success("register success")
    }).catch(e => {
      messageApi.error("register error:" + e)
    })
  }
  const deleteRight = () => {
    invoke('delete_right').then(() => {
      messageApi.success("delete success")
    }).catch(e => {
      messageApi.error("delete error:" + e)
    })
  }
  return (
    <div className="box">
      {contextHolder}
      <div>
        <p style={{ fontSize: "14px", color: '#000' }}>Select Zed executable file path:</p>
        <div style={{ display: "flex" }}>
          <Input placeholder="Please select a path"
            value={path}
          ></Input>
          <Button
            type="text"
            style={{ marginLeft: "10px" }} icon={<FolderOpenOutlined />}
            onClick={openFile}
          ></Button>
        </div>
        <div style={{ display: "flex", marginTop: '20px', justifyContent: 'space-between', paddingRight: '20px' }}>
          <Button type="primary" onClick={regRight}>Register</Button>
          <Button danger onClick={deleteRight}>Delete right register</Button>
        </div>
      </div>
    </div>
  );
}

export default App;
