import { Form, Input, message } from 'antd'
import { FormComponentProps } from 'antd/lib/form/Form'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { RouteComponentProps, withRouter } from 'react-router'

import { formItemLayout } from '../../components/form'
import Submit from '../../components/form/Submit'
import { HOME } from '../../utils'
import { httpPost } from '../../utils/request'
import Layout from './SharedLinks'

interface IProps {
  action: string,
}

const FormItem = Form.Item

class Widget extends React.Component<RouteComponentProps<any> & InjectedIntlProps & FormComponentProps & IProps> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const { form, history, intl } = this.props
    form.validateFields((err, values) => {
      if (!err) {
        httpPost(
          `/users/${this.props.action}`,
          Object.assign({}, values, { home: HOME })
        ).then(() => {
          message.success(intl.formatMessage({ id: `nut.users.${this.props.action}.success` }))
          history.push("/users/sign-in")
        }).catch(message.error)
      }
    })
  }
  public render() {
    const { formatMessage } = this.props.intl
    const { getFieldDecorator } = this.props.form
    return (<Layout title={`nut.users.${this.props.action}.title`}>
      <Form onSubmit={this.handleSubmit}>
        <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.email" />}>
          {
            getFieldDecorator('email', {
              rules: [
                {
                  message: formatMessage({ id: "form.validations.email" }),
                  required: true,
                  type: 'email',
                },
              ]
            })(<Input />)
          }
        </FormItem>
        <Submit />
      </Form>
    </Layout>)
  }
}


export default withRouter(injectIntl(Form.create()(Widget)))
