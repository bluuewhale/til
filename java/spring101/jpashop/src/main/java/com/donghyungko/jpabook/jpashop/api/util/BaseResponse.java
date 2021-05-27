package com.donghyungko.jpabook.jpashop.api.util;


import lombok.AccessLevel;
import lombok.Data;
import lombok.NoArgsConstructor;

import javax.persistence.OneToMany;
import java.util.List;

@NoArgsConstructor(access =  AccessLevel.PROTECTED)
public abstract class BaseResponse {}
